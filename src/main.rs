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
    let mut c = Config::new(); // default config to build
    args.next(); //discard application path
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
            "-s" => {
                if let Some(arg) = args.next() {
                    if let Ok(sides) = arg.parse::<u32>() {
                        c.sides = sides;
                    }
                }
            }

            "-c" => {
                if let Some(arg) = args.next() {
                    if let Ok(count) = arg.parse::<u32>() {
                        c.count = count;
                    }
                }
            }
            "-m" => {
                if let Some(arg) = args.next() {
                    if let Ok(modifier) = arg.parse::<i32>() {
                        c.modifier = modifier;
                    }
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
        println!(
            "usage: d20 [-h | --help | -a | -d | -c N | -s N | -m +|-N]
** example d20 -a -c 1 -s 20 -m +9
** Astarion rolls with advantage, one d20 with a +9 modifier"
        );
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
    sides: u32,
    count: u32,
    modifier: i32,
}

impl Config {
    fn new() -> Config {
        Config {
            h_opt: false,
            h_flag: false,
            advantage: false,
            disadvantage: false,
            sides: 20,
            count: 1,
            modifier: 0,
        }
    }
}
