use std::env;
// We must isolate ONLY these 4 tasks to main, all else should be abstracted
// It's ok to build it here, but it must be abstracted when main becomes large
// argument values (if arg parsing isn't also
//    abstracted)

fn main() {
    println!("Welcome to best dn roller of all tiem!1!");
    println!("That's not a deez nutz joke, n represents a the range [2/4/8/10/20/100]");
    // 1) Calling the command line parsing logic with
    let mut args = env::args();
    let mut c = Config::new(); // default config to build in the
    args.next(); //ignore application path
    // 2) Setting up configuration
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" => c.h_opt = true,
            "--help" => c.h_flag = true,
            "-a" => {
                c.advantage = true;
                c.disadvantage = false;
            }
            "-d" => {
                c.advantage = false;
                c.disadvantage = true;
            }
            "-test" => {
                if let Some(arg) = args.next() {
                    c.test1 = arg;
                }
            }
            "-test2" => {
                if let Some(arg) = args.next() {
                    c.test2 = arg;
                }
            }
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
    test1: String,
    test2: String,
}

impl Config {
    fn new() -> Config {
        Config {
            h_opt: false,
            h_flag: false,
            advantage: false,
            disadvantage: false,
            test1: String::new(),
            test2: String::new(),
        }
    }
}
