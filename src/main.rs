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
    println!("Please enjoy Rollin' @DN");
    let mut args = env::args();
    let mut c = Config::default();
    match args.len() {
        1 => {
            // show help and give default d20 roll
            c.h_opt = true;
            c.build(&mut args);
            rollem(&c);
        }
        // I believe I can do much better here
        2 => {
            c.build(&mut args);
            if !c.h_opt && !c.h_flag {
                rollem(&c);
            }
        }
        _ => {
            c.build(&mut args); // only display help
            rollem(&c);
        }
    }
}

fn rollem(c: &Config) {
    let roll = Roll::new(c);
    let log = roll.roll_log;
    let roll = roll.final_roll;
    let out = format!("{log} = {roll}");
    println!("{out}");
}
