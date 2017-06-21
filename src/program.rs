use ast::{Assignment, Statement, ConstDecl, WireDecl, WireWidth, WireValue, WireValues, Expr};
use errors::Error;
use std::collections::hash_set::HashSet;
use std::collections::hash_map::HashMap;
use std::collections::btree_map::BTreeMap;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::hash::Hash;

struct Graph<T> {
    edges: HashMap<T, HashSet<T>>,
    edges_inverted: HashMap<T, HashSet<T>>,
    nodes: HashSet<T>,
    num_edges: usize,
}

impl<T: Eq + Hash + Clone + Debug> Graph<T> {
    fn add_node(&mut self, node: T) {
        self.nodes.insert(node);
    }

    fn insert(&mut self, from: T, to: T) {
        debug!("insert node {:?} / {:?}", from, to);
        self.add_node(from.clone());
        self.add_node(to.clone());
        let for_node = self.edges.entry(from.clone()).or_insert_with(|| HashSet::new());
        (*for_node).insert(to.clone());
        let inverted_for_node = self.edges_inverted.entry(to).or_insert_with(|| HashSet::new());
        (*inverted_for_node).insert(from);
        self.num_edges = self.num_edges + 1;
    }

    fn contains_node(&self, node: &T) -> bool {
        return self.nodes.contains(&node);
    }

    fn contains(&self, from: T, to: T) -> bool {
        if let Some(ref out_edges) = self.edges.get(&from) {
            out_edges.contains(&to)
        } else {
            false
        }
    }

    // topological sort, or find cycle
    fn topological_sort(&self) -> Result<Vec<T>, Vec<T>> {
        let mut result = Vec::new();

        // Step 1: Queue nodes with no incoming edges
        let mut queue = VecDeque::new();
        let mut num_in_unvisited = HashMap::new();
        for node in &self.nodes {
            if !self.edges_inverted.contains_key(&node) {
                debug!("found starting node {:?}", node);
                queue.push_front(node.clone());
            }
            num_in_unvisited.insert(node.clone(),
                self.edges_inverted.get(&node).map_or(0, |x| x.len())
            );
        }

        debug!("Initial queue: {:?}", queue);
        debug!("Initial in counts: {:?}", num_in_unvisited);

        let mut visited = HashSet::new();
        while let Some(cur) = queue.pop_back() {
            debug!("Process {:?} from queue", cur);
            result.push(cur.clone());
            if let Some(out_edges) = self.edges.get(&cur) {
                for out_node in out_edges {
                    let pair = (cur.clone(), out_node.clone());
                    debug!("Found edge {:?}", pair);
                    if !visited.contains(&pair) {
                        visited.insert(pair);
                        let new_in_unvisited = num_in_unvisited.get(out_node).unwrap_or(&0) - 1;
                        num_in_unvisited.insert(out_node.clone(), new_in_unvisited);
                        if new_in_unvisited == 0 {
                            debug!("Now enqueue {:?}", out_node);
                            queue.push_front(out_node.clone());
                        }
                    }
                }
            }
        }

        if visited.len() != self.num_edges {
            // found a cycle
            unimplemented!();
        }

        Ok(result)
    }

    #[cfg(test)]
    fn in_edges(&self, to: &T) -> HashSet<T> {
        if let Some(result) = self.edges_inverted.get(to) {
            result.clone()
        } else {
            HashSet::new()
        }
    }

    #[cfg(test)]
    fn out_edges(&self, to: &T) -> HashSet<T> {
        if let Some(result) = self.edges.get(to) {
            result.clone()
        } else {
            HashSet::new()
        }
    }

    fn new() -> Graph<T> {
        Graph {
            edges: HashMap::new(),
            edges_inverted: HashMap::new(),
            nodes: HashSet::new(),
            num_edges: 0,
        }
    }
}

#[cfg(test)]
fn verify_sort<T: Eq + Clone + Hash + Debug>(graph: &Graph<T>) {
    if let Ok(the_result) = graph.topological_sort() {
        let mut seen = HashSet::new();
        for node in &the_result {
            for other in graph.out_edges(&node) {
                assert!(!seen.contains(&other), "{:?} -> {:?} violates order {:?}",
                    node, other, the_result);
            }
            seen.insert(node.clone());
        }
    } else {
        assert!(false);
    }
}


#[test]
fn test_graph() {
    let mut graph = Graph::new();
    graph.insert("foo", "bar");
    graph.insert("bar", "baz");
    verify_sort(&graph);
    graph.insert("foo", "baz");
    verify_sort(&graph);
    graph.insert("foo", "quux");
    graph.insert("quux", "other");
    verify_sort(&graph);
    graph.add_node("unused");
    verify_sort(&graph);
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Action {
    // not included:
        // register bank processing (done at beginning of cycle)
    Assign(String, Box<Expr>, WireWidth),
    ReadProgramRegister { number: String, out_port: String },
    ReadMemory { is_read: Option<String>, address: String, out_port: String, bytes: u8 },
    // these actions MUST be done last:
    WriteProgramRegister { number: String, in_port: String },
    WriteMemory { is_write: Option<String>, address: String, in_port: String, bytes: u8 },
    SetStatus { in_wire: String },
}

// psuedo-assignment for fixed functionality
pub struct FixedFunction {
    in_wires: Vec<WireDecl>,
    out_wire: Option<String>,
    action: Action,
    mandatory: bool,
    is_last: bool,
}

impl FixedFunction {
    fn read_port(number: &str, output: &str) -> FixedFunction {
        FixedFunction {
            in_wires: vec!(WireDecl { name: String::from(number), width: WireWidth::Bits(4) }),
            out_wire: Some(String::from(output)),
            action: Action::ReadProgramRegister {
                number: String::from(number),
                out_port: String::from(output),
            },
            mandatory: false,
            is_last: false,
        }
    }

    fn write_port(number: &str, input: &str) -> FixedFunction {
        FixedFunction {
            in_wires: vec!(
                WireDecl {
                    name: String::from(number),
                    width: WireWidth::Bits(4),
                },
                WireDecl {
                    name: String::from(input),
                    width: WireWidth::Bits(64),
                }
            ),
            out_wire: None,
            action: Action::WriteProgramRegister {
                number: String::from(number),
                in_port: String::from(input),
            },
            mandatory: false,
            is_last: true,
        }
    }
}

pub fn y86_fixed_functions() -> Vec<FixedFunction> {
    return vec!(
        FixedFunction {
            in_wires: vec!(WireDecl {
                name: String::from("Stat"),
                width: WireWidth::Bits(4),
            }),
            out_wire: None,
            action: Action::SetStatus { in_wire: String::from("Stat") },
            mandatory: true,
            is_last: true,
        },
        FixedFunction {
            in_wires: vec!(WireDecl {
                name: String::from("pc"),
                width: WireWidth::Bits(64),
            }),
            out_wire: Some(String::from("i10bytes")),
            action: Action::ReadMemory {
                is_read: None,
                address: String::from("pc"),
                out_port: String::from("i10bytes"),
                bytes: 10
            },
            mandatory: true,
            is_last: false,
        },
        FixedFunction {
            in_wires: vec!(WireDecl {
                name: String::from("mem_addr"),
                width: WireWidth::Bits(64),
            },
            WireDecl {
                name: String::from("mem_read"),
                width: WireWidth::Bits(1),
            }),
            out_wire: Some(String::from("mem_output")),
            action: Action::ReadMemory {
                is_read: Some(String::from("mem_read")),
                address: String::from("mem_addr"),
                out_port: String::from("mem_output"),
                bytes: 8
            },
            mandatory: false,
            is_last: false,
        },
        FixedFunction {
            // FIXME: some way to indicate that mem_write -> mem_input + mem_addr?
            in_wires: vec!(
                WireDecl {
                    name: String::from("mem_addr"),
                    width: WireWidth::Bits(64),
                },
                WireDecl {
                    name: String::from("mem_input"),
                    width: WireWidth::Bits(64),
                },
                WireDecl {
                    name: String::from("mem_write"),
                    width: WireWidth::Bits(1),
                }
            ),
            out_wire: None,
            action: Action::WriteMemory {
                is_write: Some(String::from("mem_write")),
                address: String::from("mem_addr"),
                in_port: String::from("mem_input"),
                bytes: 8,
            },
            mandatory: false,
            is_last: false,
        },
        FixedFunction::read_port("reg_srcA", "reg_outputA"),
        FixedFunction::read_port("reg_srcB", "reg_outputB"),
        FixedFunction::write_port("reg_dstE", "reg_inputE"),
        FixedFunction::write_port("reg_dstM", "reg_inputM")
    )
}

#[derive(Debug)]
pub struct RegisterBank {
    signals: Vec<(String, String, WireWidth)>, // in, out
    defaults: WireValues, // mapped to out names
    stall_signal: String,
    bubble_signal: String,
}

pub struct RegisterWritePort {
    number: String,
    value: String
}

// interpreter representation of a program
#[derive(Debug)]
pub struct Program {
    constants: WireValues,
    actions: Vec<Action>,  // in topological order
    register_banks: Vec<RegisterBank>,
}


fn resolve_constants(exprs: &HashMap<&str, &Expr>) -> Result<HashMap<String, WireValue>, Error> {
    let mut graph = Graph::new();
    for (name, expr) in exprs {
        for in_name in expr.referenced_wires() {
            graph.insert(in_name, name);
        }
        graph.add_node(name);
    }
    if let Ok(sorted) = graph.topological_sort() {
        let mut results = HashMap::new();
        for name in sorted {
            let value = try!(exprs.get(&name).unwrap().evaluate(&results));
            results.insert(
                String::from(name),
                value
            );
        }
        Ok(results)
    } else {
        unimplemented!();
    }
}

fn assignments_to_actions<'a>(
        assignments: &'a HashMap<&'a str, &Expr>,
        widths: &'a HashMap<&'a str, WireWidth>,
        known_values: &'a HashSet<&'a str>,
        fixed_functions: &'a Vec<FixedFunction>,
    ) -> Result<Vec<Action>, Error> {
    let mut fixed_by_output = HashMap::new();
    let mut fixed_no_output = Vec::new();
    let mut graph = Graph::new();
    for (name, expr) in assignments {
        graph.add_node(*name);
        for in_name in expr.referenced_wires() {
            if !known_values.contains(in_name) {
                if !assignments.contains_key(in_name) {
                    return Err(Error::UndefinedWire(String::from(in_name)));
                }
                graph.insert(in_name, name);
            }
        }
    }

    let mut unused_fixed_inputs = HashSet::new();
    let mut used_fixed_inputs = HashSet::new();
    for fixed in fixed_functions.iter() {
        let mut missing_inputs: Vec<&str> = Vec::new();
        for in_name in &fixed.in_wires {
            if known_values.contains(in_name.name.as_str()) {
                return Err(Error::RedefinedBuiltinWire(in_name.name.clone()));
            }
            if !assignments.contains_key(in_name.name.as_str()) {
                missing_inputs.push(in_name.name.as_str());
            }
        }
        if fixed.mandatory && missing_inputs.len() > 0 {
            let missing_list = missing_inputs.iter().map(|x| Error::UnsetWire(String::from(*x))).collect();
            return Err(Error::MultipleErrors(missing_list));
        } else if missing_inputs.len() > 0 {
            if let Some(ref name) = fixed.out_wire {
                if graph.contains_node(&name.as_str()) {
                    let missing_list = missing_inputs.iter().map(|x| Error::UnsetWire(String::from(*x))).collect();
                    return Err(Error::MultipleErrors(missing_list));
                }
            }
            if missing_inputs.len() != fixed.in_wires.len() {
                for in_name in &fixed.in_wires {
                    if assignments.contains_key(in_name.name.as_str()) {
                        unused_fixed_inputs.insert(in_name.name.as_str());
                    }
                }
            }
            continue;
        }
        match fixed.out_wire {
            None => {
                fixed_no_output.push(fixed);
            },
            Some(ref name) => {
                if known_values.contains(name.as_str()) ||
                   assignments.contains_key(name.as_str()) {
                    return Err(Error::RedefinedBuiltinWire(name.clone()));
                }
                fixed_by_output.insert(name.as_str(), fixed);
                for in_name in &fixed.in_wires {
                    used_fixed_inputs.insert(in_name.name.as_str());
                    graph.insert(in_name.name.as_str(), name.as_str());
                }
            }
        }
    }

    // if any piece of fixed functionality was not created because some but not all of its
    // inputs exist, trigger an error unless all those inputs are used by other fixed
    // functionality.

    // this makes doing something like setting reg_dstE without setting reg_inputE an error,
    // but doesn't make something like setting mem_addr without
    {
        let mut missing_inputs = Vec::new();
        for name in unused_fixed_inputs {
            if !used_fixed_inputs.contains(name) {
                missing_inputs.push(name);
            }
        }

        if missing_inputs.len() > 0 {
            let missing_list = missing_inputs.iter().map(
                |x| Error::UnsetWire(String::from(*x))).collect();
            return Err(Error::MultipleErrors(missing_list));
        }
    }

    let mut result = Vec::new();

    if let Ok(sorted) = graph.topological_sort() {
        // FIXME: covered is just a sanity-check, should be removeable
        let mut covered = known_values.clone();
        for name in sorted {
            match assignments.get(name) {
                Some(expr) => {
                    for in_name in expr.referenced_wires() {
                        assert!(covered.contains(&in_name));
                    }
                    if let Some(the_width) = widths.get(name) {
                        try!(try!(expr.width(widths)).combine(*the_width));
                        result.push(Action::Assign(
                            String::from(name),
                            Box::new((*expr).clone()),
                            *the_width,
                        ));
                    } else {
                        return Err(Error::UndefinedWire(String::from(name)));
                    }
                },
                None => {
                    let fixed = fixed_by_output.get(name).unwrap();
                    for in_name in &fixed.in_wires {
                        assert!(covered.contains(in_name.name.as_str()));
                    }
                    result.push(fixed.action.clone());
                }
            }
            covered.insert(name);
        }
    } else {
        unimplemented!();
    }

    for fixed in &fixed_no_output {
        result.push(fixed.action.clone());
    }

    return Ok(result);
}

impl Program {
    pub fn new_y86(statements: Vec<Statement>) -> Result<Program, Error> {
        Program::new(statements, y86_fixed_functions())
    }

    pub fn new(
        statements: Vec<Statement>,
        fixed_functions: Vec<FixedFunction>
        // TODO: preamble (constants)
    ) -> Result<Program, Error> {
        // Step 1: Split statements into constant declarations, wire declarations, assignments
        let mut constants_raw: HashMap<&str, &Expr> = HashMap::new();
        let mut wires = HashMap::new();
        let mut needed_wires = HashSet::new();
        let mut assignments = HashMap::new();
        let mut register_banks_raw = Vec::new();
        for fixed in &fixed_functions {
            for ref in_wire in &fixed.in_wires {
                wires.insert(in_wire.name.as_str(), in_wire.width);
            }
        }
        // FIXME: detect duplicates somewhere here
        for statement in &statements {
            match *statement {
                Statement::ConstDecls(ref decls) => {
                    for ref decl in decls {
                        constants_raw.insert(decl.name.as_str(), &*decl.value);
                    }
                },
                Statement::WireDecls(ref decls) => {
                    for ref decl in decls {
                        wires.insert(decl.name.as_str(), decl.width);
                        needed_wires.insert(decl.name.as_str());
                    }
                },
                Statement::Assignment(ref assign) => {
                    for name in &assign.names {
                        // FIXME: detect multiple declarations
                        assignments.insert(name.as_str(), &*assign.value);
                    }
                },
                Statement::RegisterBankDecl(ref decl) => {
                    register_banks_raw.push(decl);
                }
                _ => unimplemented!(),
            }
        }

        debug!("const decls: {:?}", constants_raw);
        debug!("wire decls: {:?}", wires);
        debug!("assignments: {:?}", assignments);

        // Step 2: find constants values
        let constants = try!(resolve_constants(&constants_raw));

        // Step 3: resolve register banks
        let mut register_banks = Vec::new();
        for decl in &register_banks_raw {
            // FIXME: should really iterate over graphemes
            let name_chars: Vec<char> = decl.name.chars().collect();
            if name_chars.len() != 2 {
                return Err(Error::InvalidRegisterBankName(decl.name.clone()));
            }
            let in_prefix = name_chars[0];
            let out_prefix = name_chars[1];
            if !in_prefix.is_lowercase() || !out_prefix.is_uppercase() {
                return Err(Error::InvalidRegisterBankName(decl.name.clone()));
            }
            let mut signals = Vec::new();
            let mut defaults = HashMap::new();
            let mut stall_signal = String::from("stall_");
            stall_signal.push(out_prefix);
            let mut bubble_signal = String::from("bubble_");
            bubble_signal.push(out_prefix);
            for register in &decl.registers {
                let mut in_name = String::new();
                let mut out_name = String::new();
                in_name.push(in_prefix);
                out_name.push(out_prefix);
                in_name.push('_');
                out_name.push('_');
                in_name.push_str(register.name.as_str());
                out_name.push_str(register.name.as_str());
                // FIXME: better errors if failure here
                let value = register.default.evaluate(&constants)?;
                // FIXME: better error
                value.width.combine(register.width)?;
                defaults.insert(out_name.clone(), value.as_width(register.width));
                debug!("Generated wires {} and {} for register", in_name, out_name);
                signals.push((in_name, out_name, register.width));
            }
            // FIXME: detect redefinition of signals
            register_banks.push(RegisterBank {
                signals: signals,
                defaults: defaults,
                stall_signal: stall_signal,
                bubble_signal: bubble_signal,
            });
        }

        let actions = {
            // create nonmutable reference so we can borrow strings from register banks
            let register_banks = &register_banks;
            // move wires and needed_wires so they are dropped before register_banks
            let mut wires = wires;
            let mut needed_wires = needed_wires;
            
            // track widths, values we do/don't need assignment statements for
            let mut known_values = HashSet::new();
            for bank in register_banks {
                for signal in &bank.signals {
                    let in_name = &signal.0;
                    let out_name = &signal.1;
                    let width = signal.2;
                    wires.insert(out_name.as_str(), width);
                    known_values.insert(out_name.as_str());
                    wires.insert(in_name.as_str(), width);
                    needed_wires.insert(in_name.as_str());
                }
                wires.insert(bank.stall_signal.as_str(), WireWidth::Bits(1));
                wires.insert(bank.bubble_signal.as_str(), WireWidth::Bits(1));
            }

            // Step 4: Check for missing wires
            for name in needed_wires {
                if !assignments.contains_key(name) {
                    return Err(Error::UnsetWire(String::from(name)));
                }
            }

            // Step 5: order remaining assignments
            for key in constants_raw.keys() {
                known_values.insert(*key);
                wires.insert(key, constants.get(&String::from(*key)).unwrap().width);
            }

            assignments_to_actions(&assignments, &wires,
                                   &known_values, &fixed_functions)?
        };

        Ok(Program {
            constants: constants,
            actions: actions,
            register_banks: register_banks,
        })
    }

    pub fn constants(&self) -> WireValues {
        self.constants.clone()
    }

    pub fn initial_state(&self) -> WireValues {
        let mut values = self.constants();
        for bank in &self.register_banks {
            for signal in &bank.signals {
                let in_name = &signal.0;
                let out_name = &signal.1;
                let the_value = *bank.defaults.get(out_name).unwrap();
                values.insert(in_name.clone(), the_value);
                values.insert(out_name.clone(), the_value);
            }
            values.insert(bank.bubble_signal.clone(), WireValue::false_value());
            values.insert(bank.stall_signal.clone(), WireValue::false_value());
        }
        values
    }

    fn process_register_banks(&self, values: &mut WireValues) -> Result<(), Error> {
        for bank in &self.register_banks {
            let stalled = values.get(&bank.stall_signal).unwrap().is_true();
            let bubbled = values.get(&bank.bubble_signal).unwrap().is_true();
            // FIXME: correct stall + bubble behavior
            if bubbled {
                debug!("bubble {}", bank.bubble_signal);
                for (k, v) in &bank.defaults {
                    *values.get_mut(k).unwrap() = *v;
                }
            } else if !stalled {
                for signal in &bank.signals {
                    let in_name = &signal.0;
                    let out_name = &signal.1;
                    debug!("copy {} -> {}", in_name, out_name);
                    let new_value = *values.get(in_name).unwrap();
                    *values.get_mut(out_name).unwrap() = new_value;
                }
            }
        }
        Ok(())
    }

    pub fn step_in_place(&self, values: &mut WireValues) -> Result<(), Error> {
        self.process_register_banks(values)?;
        for action in &self.actions {
            debug!("processing action {:?}", action);
            match action {
               &Action::Assign(ref name, ref expr, ref width) => {
                  let result = try!(expr.evaluate(values)).as_width(*width);
                  debug!("computed value {:?}", result);
                  let mut inserted = false;
                  if let Some(value) = values.get_mut(name) {
                      *value = result;
                      inserted = true;
                  }
                  if !inserted {
                      values.insert(name.clone(), result);
                  }
               },
               _ => unimplemented!(),
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Memory {
    data: BTreeMap<u64, u8>
}

impl Memory {
    pub fn new() -> Memory {
        Memory { data: BTreeMap::new() }
    }
}

#[derive(Debug)]
pub struct RunningProgram {
    program: Program,
    cycle: u32,
    values: WireValues,
    memory: Memory,
    registers: Vec<u64>,
    zero_register: usize,
}

impl RunningProgram {
    pub fn new(program: Program,
               num_registers: usize,
               zero_register: usize) -> RunningProgram {
        let values = program.initial_state();
        let mut registers = Vec::new();
        for i in 0..num_registers {
            registers.push(0);
        }
        RunningProgram {
            program: program,
            cycle: 0,
            values: values,
            memory: Memory::new(),
            registers: registers,
            zero_register: zero_register,
        }
    }

    pub fn new_y86(program: Program) -> RunningProgram {
        RunningProgram::new(
            program,
            16,
            15
        )
    }

    pub fn cycle(&self) -> u32 { self.cycle }

    pub fn values(&self) -> &WireValues { &self.values }

    pub fn step(&mut self) -> Result<(), Error> {
        let program = &self.program;
        try!(program.step_in_place(&mut self.values));
        self.cycle += 1;
        Ok(())
    }

    pub fn done(&self) -> bool {
        self.values.get("Stat").unwrap_or(&WireValue::from_u64(1)) != &WireValue::from_u64(1)
    }

    pub fn dump(&self) -> String {
        format!("{:?}", self.values)
    }
}