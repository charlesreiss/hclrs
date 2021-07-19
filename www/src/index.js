import { 
    wasm_y86_hcl_to_file_contents,
    wasm_parse_y86_hcl,
    wasm_setup_program_y86,
    wasm_run_y86,
    wasm_new_run_options,
    wasm_dump_y86,
    wasm_step,
    wasm_running_program_state_as_json,
    FileContents,
    Program,
    RunningProgram
} from '../../pkg'

export async function hclrs_start(hcl_contents, memory_contents, run_options) {
    const program = await wasm_parse_y86_hcl(hcl_contents)
    console.log("program = " + program + "; memory_contents = " + memory_contents + "; run_options = " + run_options);
    const running_program = await wasm_setup_program_y86(program, memory_contents, run_options)
    return running_program
}

export async function hclrs_run(hcl_program, memory_contents) {
    const hcl_contents = await wasm_y86_hcl_to_file_contents(hcl_program, "<web>")
    const run_options = await wasm_new_run_options();
    const running_program = hclrs_start(hcl_contents, memory_contents, run_options)
    const output = await wasm_run_y86(running_program, contents)
    return {
        output: output,
        finalOutput: wasm_dump_y86(running_program),
    }
}

async function do_run() {
    try {
        var code = document.getElementById("code");
        var memory = document.getElementById("memory");
        var result = await hclrs_run(
            document.getElementById("code").value, 
            document.getElementById("memory").value, 
        );
        document.getElementById("output").textContent = result.output;
        document.getElementById("final-output").textContent = result.finalOutput;
    } catch (error) {
        document.getElementById("output").textContent = "<failed to run>";
        document.getElementById("final-output").textContent = error;
    }
}

var current_program;
var current_hcl_contents;
var current_history = [];

function populate_register_table(tbl, history_cell) {
    while (tbl.rows.length > 0) tbl.deleteRow(-1);
    history_cell.registers.forEach( (value, index) => {
        var row = tbl.insertRow();
        row.insertCell().textContent = "R" + index;
        row.insertCell().textContent = value;
    })
}

function populate_wires_table(tbl, history_cell) {
    while (tbl.rows.length > 0) tbl.deleteRow(-1);
    var constants = history_cell.constants;
    var register_signals = {}
    history_cell.register_banks.forEach( bank => {
        bank.inputs.forEach(x => { register_signals[x] = 1; })
        bank.outputs.forEach(x => { register_signals[x] = 1; })
    })
    var keys = Object.keys(history_cell.wires)
    keys.sort()
    keys.forEach( key => {
        var value = history_cell.wires[key];
        if (!constants[key] /* && !register_signals[key] */) {
            var row = tbl.insertRow();
            row.insertCell().textContent = key;
            row.insertCell().textContent = value;
        }
    })
}

function show_current_step() {
    var index = document.getElementById("current-step").value;
    populate_register_table(
        document.getElementById("regfile-registers"),
        current_history[index].state
    );
    populate_wires_table(
        document.getElementById("wires"),
        current_history[index].state,
    );
    document.getElementById("output").textContent = current_history[index].output;
}

function show_history(index) {
    document.getElementById("current-step").value = index;
    show_current_step();
}

async function update_generic() {
    document.getElementById("raw-history").textContent = JSON.stringify(current_history, null, 2);
    document.getElementById("max-step").textContent = current_history.length - 1;
    document.getElementById("current-step").disabled = false;
}

async function do_start() {
    console.log("START");
    try {
        var code = document.getElementById("code").value;
        current_hcl_contents = await wasm_y86_hcl_to_file_contents(code, "<web>");
        console.log("current_hcl_contents = " + current_hcl_contents)
        console.log("instanceof FileContnets: " + (current_hcl_contents instanceof FileContents))
        var run_options = await wasm_new_run_options();
        var memory_contents = document.getElementById("memory").value;
        current_program = await hclrs_start(current_hcl_contents, memory_contents, run_options);
        current_history.push({
           output: '',
           state: wasm_running_program_state_as_json(current_program),
        })
        console.log("current_program = " + current_program);
        document.getElementById("output").textContent = "<at beginning>";
        document.getElementById("final-output").textContent = await wasm_dump_y86(current_program);
    } catch (error) {
        document.getElementById("output").textContent = "<failed to start>";
        document.getElementById("final-output").textContent = error;
    }
    console.log("about to show history with length = " + (current_history.length));
    console.log("current_history[0] = " + current_history[0]);
    show_history(current_history.length - 1);
}

async function do_step() {
    console.log("STEP");
    console.log("current_program = " + current_program + "; current_hcl_contents = " + current_hcl_contents);
    console.log("instanceof FileContnets: " + (current_hcl_contents instanceof FileContents))
    console.log("instanceof RunningProgram: " + (current_program instanceof RunningProgram))
    var current_step = {}
    try {
        current_step.output = wasm_step(current_program, current_hcl_contents);
        current_step.state = wasm_running_program_state_as_json(current_program);
        current_step.dump  = wasm_dump_y86(current_program);
    } catch (error) {
        current_step.state = current_program.state_as_json();
        current_step.error = error;
        current_step.dump  = wasm_dump_y86(current_program);
    }
    current_history.push(current_step);
    if (current_step.error) {
        document.getElementById("final-output").textContent = current_step['error']
    } else {
        document.getElementById("final-output").textContent = current_step['dump']
    }
    await update_generic();
    show_history(current_history.length - 1);
}

async function handle_yo_file() {
    const theFile = this.files[0];
    document.getElementById("memory").value = await theFile.text();
}

async function handle_hcl_file() {
    const theFile = this.files[0];
    document.getElementById("code").value = await theFile.text();
}

const runButton = document.getElementById("run-button");
runButton.addEventListener('click', () => do_run())
const startButton = document.getElementById("start-button");
startButton.addEventListener('click', () => do_start())
const stepButton = document.getElementById("step-button");
stepButton.addEventListener('click', () => do_step())
const yoFileUpload = document.getElementById("memory-file");
yoFileUpload.addEventListener('change', handle_yo_file);
const hclFileUpload = document.getElementById("code-file");
hclFileUpload.addEventListener('change', handle_hcl_file);
const currentStepInput = document.getElementById("current-step");
currentStepInput.addEventListener('change', show_current_step);
