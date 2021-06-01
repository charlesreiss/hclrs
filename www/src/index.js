import { 
    wasm_y86_hcl_to_file_contents,
    wasm_parse_y86_hcl,
    wasm_setup_program_y86,
    wasm_run_y86
} from '../../pkg'

export async function hclrs_run(hcl_program, y86) {
    const contents = await wasm_y86_hcl_to_file_contents(hcl_program, "<web>")
    const program = await wasm_parse_y86_hcl(contents)
    const running_program = await wasm_setup_program_y86(program)
    const output = await wasm_run_y86(running_program)
    return output;
}

hclrs_run("","").then(x => alert(x))
