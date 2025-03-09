use std::env;
use std::process;

fn main() {
    println!("Welcome to best d20 roller of all tiem!1!");
    // We must isolate ONLY these 4 tasks to main, all else should be abstracted
    // It's ok to build it here, but it must be abstracted when main becomes large
    // 1) Calling the command line parsing logic with argument values (if arg parsing isn't also
    //    abstracted)
    // Kayla big brain stuffs 
    // let args = env::args();
    // let c = Config::new(); // default config to build in the
    // while below
    // while let Some(arg) = args.next() {}
    let args = env::args().collect();
    let c = Config::build(args).unwrap_or_else(|err| {
        // THING 1.2 we need to isolate and add
        // detail to error handling as a whole
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    dbg!(c.c1);

    // 2) Setting up configuration
    // 3) Calling a run function in lib.rs
    // 4) Handling errors if run returns error
}

struct Config {
    c1: String,
}

impl Config {
    fn build(args: Vec<String>) -> Result<Config, &'static str> {
        let c1 = args[1].to_string();
        Ok(Config { c1 })
    }
}
