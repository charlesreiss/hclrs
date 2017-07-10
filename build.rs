extern crate lalrpop;
use std::env;

fn main() {
    // FIXME: workaround for lalrpop issue #240; should eventually be removed.
    env::set_var("LALRPOP_LANE_TABLE", "disabled");
    lalrpop::process_root().unwrap();
}
