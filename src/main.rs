// use std::env;

fn main() {
    // MVP in notebook
    // 1)Ask user for #d20 to roll
    // 2)Give user quit option
    // 3)Handle input errors gracefully
    // 4)display roll total, and individual rolls in brackets
    // 5)80% code coverage in test framework
    //
    // MVP 2.0
    // logging to text
    // accept cli args
    // let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    // env::args good enough?  Need args_os robustosity?
    // ex: d20 6d20 adv mod
    // manual cli at first
    // then on to crates.io to find something that already exists
    // MVP 3.0
    // real logging system
    //
    // Task 1: Lay scaffold for testing
    // Task 2: Display greeting
    // Task 3: Capture user input
    // Task 4: display results of roll
    // Task 5: Ask for next roll/quit
    println!("d20 DN!");
    // We must isolate ONLY these 4 tasks to main, all else should be abstracted
    // It's ok to build it here, but it must be abstracted when main becomes large
    // 1) Calling the command line parsing logic with argument values (if arg parsing isn't also
    //    abstracted)
    // 2) Setting up configuration
    // 3) Calling a run function in lib.rs
    // 4) Handling errors if run returns error
}
