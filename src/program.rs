use ast::{Assignment, Statement, ConstDecl, WireDecl, WireValue, WireValues, Expr};
use errors::Error;
use std::collections::hash_set::HashSet;
use std::collections::hash_map::HashMap;
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

#[derive(Debug)]
pub enum Action {
    Assign(String, Box<Expr>),
    // TODO: fixed functionality
}

// interpreter representation of a program
#[derive(Debug)]
pub struct Program {
    constants: WireValues,
    actions: Vec<Action>,  // in topological order
    // FIXME: register banks
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

fn assignments_to_actions(
        assignments: &HashMap<&str, &Expr>,
        known_values: &HashSet<&str>
    ) -> Result<Vec<Action>, Error> {
    let mut graph = Graph::new();
    for (name, expr) in assignments {
        for in_name in expr.referenced_wires() {
            if !known_values.contains(in_name) {
                graph.insert(in_name, name);
            }
        }
    }

    let mut result = Vec::new();

    if let Ok(sorted) = graph.topological_sort() {
        let mut covered = known_values.clone();
        for name in sorted {
            let expr = assignments.get(name).unwrap();
            for in_name in expr.referenced_wires() {
                if !covered.contains(&in_name) {
                    if !assignments.contains_key(&in_name) {
                        return Err(Error::UndefinedWire(String::from(in_name)));
                    }
                }
            }
            result.push(Action::Assign(String::from(name), Box::new((*expr).clone())));
            covered.insert(name);
        }
    } else {
        unimplemented!();
    }

    return Ok(result);
}

impl Program {
    pub fn new(
        statements: Vec<Statement>,
        // TODO: parameterized fixed functionality/preamble
    ) -> Result<Program, Error> {
        // Step 1: Split statements into constant declarations, wire declarations, assignments
        let mut constants_raw: HashMap<&str, &Expr> = HashMap::new();
        let mut wires = HashMap::new();
        let mut assignments = HashMap::new();
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
                    }
                },
                Statement::Assignment(ref assign) => {
                    for name in &assign.names {
                        assignments.insert(name.as_str(), &*assign.value);
                    }
                }
            }
        }

        debug!("const decls: {:?}", constants_raw);
        debug!("wire decls: {:?}", wires);
        debug!("assignments: {:?}", assignments);

        // Step 2: find constants values
        let constants = try!(resolve_constants(&constants_raw));

        // Step 3: order remaining assignments
        let mut known_values = HashSet::new();
        for key in constants_raw.keys() {
            known_values.insert(*key);
        }
        let actions = try!(assignments_to_actions(&assignments, &known_values));
        Ok(Program {
            constants: constants,
            actions: actions,
        })
    }

    pub fn constants(&self) -> WireValues {
        self.constants.clone()
    }

    pub fn step_in_place(&self, values: &mut WireValues) -> Result<(), Error> {
        for action in &self.actions {
            match action {
               &Action::Assign(ref name, ref expr) => {
                  let result = try!(expr.evaluate(values));
                  // FIXME: avoid creating new string for speed?
                  values.insert(name.clone(), result);
               }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct RunningProgram {
    program: Program,
    cycle: u32,
    values: WireValues,
}

impl RunningProgram {
    pub fn new(program: Program) -> RunningProgram {
        let constants = program.constants();
        RunningProgram {
            program: program,
            cycle: 0,
            values: constants,
        }
    }

    pub fn cycle(&self) -> u32 { self.cycle }

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
