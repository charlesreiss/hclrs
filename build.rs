extern crate lalrpop;
use std::env;

fn main() {
    // FIXME: workaround for lalrpop issue #240; should eventually be removed.
    env::set_var("LALRPOP_LANE_TABLE", "disabled");
    lalrpop::Configuration::new().emit_comments(true).emit_report(true).process_current_dir().unwrap();
}
