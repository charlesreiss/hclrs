use ast::{Assignment, WireValue, WireValues, Error};

// interpreter representation of a program
#[derive(Debug)]
pub struct Program {
    constants: HashMap<String, WireValue>
    assignments: Vec<Assignment>,  // in topological order
    // FIXME: register banks
}

impl Program {
    pub fn new(
        statements: Vec<Statement>
    ) -> Result<Program, Error> {
        unimplemented!();
    }

    pub fn initial_state(&self) -> WireValues {
        unimplemented!();
    }

    pub fn step(&self, input: WireValues) -> WireValues {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct RunningProgram {
    program: Program,
    cycle: u32,
    wires: WireValues,
}

impl RunningProgram {
    pub fn new(program: Program) -> RunningProgram {
        unimplemented!();
    }

    pub fn step(&mut self) {
        unimplemented!();
    }

    pub fn done(&self) -> bool {
        unimplemented!();
    } 

    pub fn dump() -> String {
        unimplemented!();
    }
}
