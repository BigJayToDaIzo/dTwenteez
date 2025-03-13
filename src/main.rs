mod config;
mod roll;

use config::Config;
use roll::Roll;
use std::env;

// We must isolate ONLY these 4 tasks to main, all else should be abstracted
// 1) Calling the command line parsing logic with
// 2) Setting up configuration (ABSTRACT THIS NAO)
// 3) Calling a run function in lib.rs
// 4) Handling errors if run returns error
#[cfg(not(tarpaulin_include))]
fn main() {
    println!("We hope you enjoy the best dicer: Rollin' @DN");
    let mut args = env::args();
    let mut c = Config::default();
    match args.len() {
        1 => {
            // show help and give default d20 roll
            c.h_opt = true;
            let roll = Roll::new(&c);
            let log = roll.roll_log;
            let roll = roll.final_roll;
            let out = format!("{log} = {roll}");
            println!("{out}");
        }
        2 => {
            c.build(&mut args); // only display help
        }
        _ => {}
    }
}
