extern crate lalrpop;
use std::env;

fn main() {
    lalrpop::Configuration::new().emit_comments(true).emit_report(true).process_current_dir().unwrap();
}
