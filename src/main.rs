use std::env;

fn main() {
    println!("Welcome to best d20 roller of all tiem!1!");
    // We must isolate ONLY these 4 tasks to main, all else should be abstracted
    // It's ok to build it here, but it must be abstracted when main becomes large
    // 1) Calling the command line parsing logic with
    // argument values (if arg parsing isn't also
    //    abstracted)
    let mut args = env::args();
    let mut c = Config::new(); // default config to build in the
    // while below
    args.next(); //ignore application path
    // 2) Setting up configuration
    for arg in args {
        match arg.as_str() {
            "-h" => c.h_opt = true,
            "--help" => c.h_flag = true,
            "-a" => {
                c.advantage = true;
                c.disadvantage = false;
            }
            "-d" => c.disadvantage = true,
            x => {
                println!("arg string broken with invalid argument: {x}");
                c.h_opt = true;
            }
        }
    }
    if c.h_opt || c.h_flag {
        // this will get much larger and need to be abstracted
        println!("usage: d20 [-h | --help | -a | -d]");
    }
    dbg!(&c);
    // 3) Calling a run function in lib.rs
    // 4) Handling errors if run returns error
}

#[derive(Debug)]
struct Config {
    h_opt: bool,
    h_flag: bool,
    advantage: bool,
    disadvantage: bool,
}

impl Config {
    fn new() -> Config {
        Config {
            h_opt: false,
            h_flag: false,
            advantage: false,
            disadvantage: false,
        }
    }
}
