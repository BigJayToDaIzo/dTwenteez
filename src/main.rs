use d20::Config;
use std::env;

// We must isolate ONLY these 4 tasks to main, all else should be abstracted
fn main() {
    println!("We hope you enjoy the best dicer: Rollin' @DN");
    println!("That's not a deez nutz joke, N represents the range [2/4/8/10/20/100]");
    // 1) Calling the command line parsing logic with
    let mut args = env::args();
    let mut c = Config::default(); // default config to build
    // 2) Setting up configuration (ABSTRACT THIS NAO)
    c.build(&mut args);
    // let roll = c.roll();
    // 3) Calling a run function in lib.rs
    // 4) Handling errors if run returns error
}
