use config::Config;
use roll::Roll;
use std::{env, io};

mod config;
mod roll;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("Please enjoy Rollin' @DN");
    let mut args = env::args();
    let mut c = Config::default();
    match args.len() {
        1 => {
            // no arg is interactive mode!
            println!("Also please enjoy interactive rolling mode!");
            println!("pass args to roll, 'h' for help, and 'q' to quit");
            println!("examples: d6, 2d10-1, 3d6+1 d, d20 a");
            loop {
                let mut raw_args = String::new();
                println!("wut dem args iz?");
                // ex: d10
                // ex: d20+1 a
                // ex: 2d6-1 d
                match io::stdin().read_line(&mut raw_args) {
                    Ok(_) => (),
                    Err(e) => {
                        panic!("Something borkied: {e}");
                    }
                }
                // here we know raw_args read some number of bytes
                raw_args = raw_args.trim().to_string();
                c.interact(raw_args);
                rollem(&c);
            }
        }
        2 => {
            // no loop cuz args passed
            c.build(&mut args);
            if !c.h_opt && !c.h_flag {
                rollem(&c);
            }
        }
        _ => {
            c.build(&mut args); // only display help
            // rollem(&c);
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
