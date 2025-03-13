mod config;
mod roll;

use config::Config;
use roll::Roll;
use std::env;

// We must isolate ONLY these 4 tasks to main, all else should be abstracted
#[cfg(not(tarpaulin_include))]
fn main() {
    println!("We hope you enjoy the best dicer: Rollin' @DN");
    println!("That's not a deez nutz joke, N represents the range [2/4/8/10/20/100]");
    // 1) Calling the command line parsing logic with
    let mut args = env::args();
    let mut c = Config::default(); // default config to build
    // 2) Setting up configuration (ABSTRACT THIS NAO)
    c.build(&mut args);
    let r = Roll::new(&c);
    dbg!(&r.final_roll);
    dbg!(&r.display);
    // 3) Calling a run function in lib.rs
    // 4) Handling errors if run returns error
}
