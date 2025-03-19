use std::process;

#[derive(Debug)]
pub struct Config {
    pub h_opt: bool,
    pub h_flag: bool,
    pub advantage: bool,
    pub disadvantage: bool,
    pub sides: u32,
    pub count: u32,
    pub modifier: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Config {
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
    pub fn reset(&mut self) {
        self.h_opt = false;
        self.h_flag = false;
        self.advantage = false;
        self.disadvantage = false;
        self.sides = 20;
        self.count = 1;
        self.modifier = 0;
    }
    pub fn build<T>(&mut self, args: &mut T)
    where
        T: Iterator<Item = String>,
    {
        args.next(); //discard application path
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-h" => self.h_opt = true,
                "--help" => self.h_flag = true,
                "-a" => {
                    self.advantage = true;
                    self.disadvantage = false;
                }
                "-d" => {
                    self.advantage = false;
                    self.disadvantage = true;
                }
                "-s" => {
                    if let Some(arg) = args.next() {
                        if let Ok(sides) = arg.parse::<u32>() {
                            self.sides = sides;
                        } else {
                            println!("WARN: invalid argument passed to -s flag");
                            println!("unable to parse integer from {arg}");
                            println!("default value for sides of 20 was configured");
                        }
                    }
                }
                "-c" => {
                    if let Some(arg) = args.next() {
                        if let Ok(count) = arg.parse::<u32>() {
                            self.count = count;
                        } else {
                            println!("WARN: invalid argument passed to -c flag");
                            println!("unable to parse integer from {arg}");
                            println!("default value for count of 1 was configured");
                        }
                    }
                }
                "-m" => {
                    if let Some(arg) = args.next() {
                        if let Ok(modifier) = arg.parse::<i32>() {
                            self.modifier = modifier;
                        } else {
                            println!("WARN: invalid argument passed to -m flag");
                            println!("unable to parse integer from {arg}");
                            println!("default value for modifier of 0 was configured");
                        }
                    }
                }
                x => {
                    println!("arg string broken with invalid argument: {x}");
                    self.h_opt = true;
                }
            }
        }
        if self.h_opt || self.h_flag {
            // this will get much larger and need to be abstracted
            Self::display_cli_help();
        }
    }
    pub fn display_cli_help() {
        println!(
            "usage: dTwenteez [-h | --help | -a | -d | -c N | -s N | -m +|-N]
usage: `dTwenteez` (no args) will start an interactive mode 
** defaults: -c = 1, -s = 20 -m = 0 -a & -d = false
** example dTwenteez -a -m 9 ** Astarion rolls with advantage, one d20 with a +9 modifier
** example dTwenteez -d -c 3 -s 6 -m -2 ** Astarion rolls with disadvantage, 3d6 with a -2 modifier
Flags and args can be passed in any order.
Flags -c, -s and -m will expect to be followed by a space and integer argument following them.
** example dTwenteez -m -3 -a -s 6 -c 4 ** Astarion rolls with advantage, 4d6 with -3 modifier"
        );
    }
    pub fn display_interactive_help() {
        println!(
            "syntax: [n]dS[+|-M] [a|d]
Where n is optional die multiplier integer.
S is required integer for die sides.
+M or -M where M is an integer for positive or negative roll modifiers.
a or d are optional advantage or disadvantage modifiers.
examples:
2d10-3 a ** Astarion rolls 2d10 with advantage and a -3 modifier
6d6+2 d ** Astarion rolls 6d6 with disadvantage and +2 modifier
d20 ** Astarion rolls 1d20 with no modifiers of any kind"
        );
    }
    pub fn interact(&mut self, args: String) {
        self.reset();
        let args_v: Vec<&str> = args.split_whitespace().collect();
        match args_v.len() {
            1 => {
                let mut arg = args_v[0];
                if arg.contains("h") {
                    self.h_opt = true;
                    Self::display_interactive_help();
                    return;
                } else if arg.contains("q") {
                    println!("Thanks for rollin' with dTwenteez!");
                    process::exit(0);
                }
                if arg.contains('+') {
                    let arg_split: Vec<&str> = arg.split('+').collect();
                    self.modifier = arg_split[1].parse::<i32>().unwrap_or_else(|_| {
                        println!("WARN: Invald argument passed to modifier.");
                        println!("WARN: Default modifier of 0 set.");
                        0
                    });
                    arg = arg_split[0];
                }
                if arg.contains('-') {
                    let arg_split: Vec<&str> = arg.split('-').collect();
                    self.modifier = -arg_split[1].parse::<i32>().unwrap_or_else(|_| {
                        println!("WARN: Invald argument passed to modifier.");
                        println!("WARN: Default modifier of 0 set.");
                        0
                    });
                    arg = arg_split[0];
                    let die_split: Vec<&str> = arg.split('d').collect();
                    // we split on + or - and if we DO split on - pass negative value
                    if !die_split[0].is_empty() {
                        self.count = die_split[0].parse::<u32>().unwrap_or_else(|_| {
                            println!("WARN: Invalid argument passed to count.");
                            println!("WARN: Default count of 1 set.");
                            1
                        });
                    }
                    self.sides = die_split[1].parse::<u32>().unwrap_or_else(|_| {
                        println!("WARN: Invalid argument passed to sides.");
                        println!("WARN: Default sides of 20 set.");
                        20
                    });
                }
            }
            2 => {
                match args_v[1] {
                    "a" => {
                        self.advantage = true;
                        self.disadvantage = false;
                    }
                    "d" => {
                        self.disadvantage = true;
                        self.advantage = false;
                    }
                    _ => {
                        println!("WARN: invalid arg passed, only 'a' and 'd' are valid options");
                        println!("WARN: default values for adv/disadvantage set to false.");
                        self.advantage = false;
                        self.disadvantage = false;
                    }
                };
                let mut arg = args_v[0];
                if arg.contains('+') {
                    let arg_split: Vec<&str> = arg.split('+').collect();
                    self.modifier = arg_split[1].parse::<i32>().unwrap_or_else(|_| {
                        println!("WARN: Invald argument passed to modifier.");
                        println!("WARN: Default modifier of 0 set.");
                        0
                    });
                    arg = arg_split[0];
                }
                if arg.contains('-') {
                    let arg_split: Vec<&str> = arg.split('-').collect();
                    self.modifier = -arg_split[1].parse::<i32>().unwrap_or_else(|_| {
                        println!("WARN: Invald argument passed to modifier.");
                        println!("WARN: Default modifier of 0 set.");
                        0
                    });
                    arg = arg_split[0];
                }
                let die_split: Vec<&str> = arg.split('d').collect();
                // we split on + or - and if we DO split on - pass negative value
                if !die_split[0].is_empty() {
                    self.count = die_split[0].parse::<u32>().unwrap_or_else(|_| {
                        println!("WARN: Invalid argument passed to count.");
                        println!("WARN: Default count of 1 set.");
                        1
                    });
                }
                self.sides = die_split[1].parse::<u32>().unwrap_or_else(|_| {
                    println!("WARN: Invalid argument passed to sides.");
                    println!("WARN: Default sides of 20 set.");
                    20
                });
            }
            _ => {
                println!("WARN: wrong number of args passed.");
                println!("Rolling default 1d20");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_config(args: Vec<&str>) -> Config {
        let mut c = Config::default();
        let mut args = args.iter().map(|s| s.to_string());
        c.build(&mut args);
        c
    }

    #[test]
    fn new_works() {
        let c = Config::new();
        assert_eq!(c.sides, 20);
        assert_eq!(c.count, 1);
        assert_eq!(c.modifier, 0);
    }
    #[test]
    fn build_works() {
        let c = build_config(vec![""]);
        assert!(!c.h_opt);
    }
    #[test]
    fn help_opts_work() {
        let c = build_config(vec!["", "-h", "--help"]);
        assert!(c.h_opt);
        assert!(c.h_flag);
    }
    #[test]
    fn advantage_works() {
        let c = build_config(vec!["", "-h", "--help", "-a"]);
        assert!(c.h_opt);
        assert!(c.h_flag);
        assert!(c.advantage);
        assert!(!c.disadvantage);
    }
    #[test]
    fn disadvantage_works() {
        let c = build_config(vec!["", "-d"]);
        assert!(c.disadvantage);
        assert!(!c.advantage);
    }
    #[test]
    fn sides_works() {
        let c = build_config(vec!["", "-d", "-s", "10"]);
        assert!(c.disadvantage);
        assert_eq!(c.sides, 10);
    }
    #[test]
    fn count_works() {
        let c = build_config(vec!["", "-s", "10", "-c", "2"]);
        assert_eq!(c.sides, 10);
        assert_eq!(c.count, 2);
    }
    #[test]
    fn pos_modifier_works() {
        let c = build_config(vec!["", "-c", "2", "-m", "13"]);
        assert_eq!(c.count, 2);
        assert_eq!(c.modifier, 13);
    }
    #[test]
    fn neg_modifier_works() {
        let c = build_config(vec!["", "-c", "2", "-m", "-13"]);
        assert_eq!(c.count, 2);
        assert_eq!(c.modifier, -13);
    }
    #[test]
    fn invalid_flag_warns_user_but_continues() {
        let c = build_config(vec!["", "-42069", "-s", "10", "-c", "2"]);
        assert_eq!(c.sides, 10);
        assert_eq!(c.count, 2);
        assert!(c.h_opt);
    }
    #[test]
    fn invalid_arg_passed_to_sides_sets_default() {
        let c = build_config(vec!["", "-s", "abc"]);
        assert_eq!(c.sides, 20);
    }
    #[test]
    fn invalid_arg_passed_to_count_sets_default() {
        let c = build_config(vec!["", "-c", "abc"]);
        assert_eq!(c.count, 1);
    }
    #[test]
    fn invalid_arg_passed_to_modifier_sets_default() {
        let c = build_config(vec!["", "-m", "abc"]);
        assert_eq!(c.modifier, 0);
    }
    #[test]
    fn interactive_d10_works() {
        let mut c = Config::default();
        c.interact("d10".to_string());
        assert_eq!(c.sides, 10);
    }
    #[test]
    fn interactive_pos_modifier_works() {
        let mut c = Config::default();
        c.interact("d10+1".to_string());
        assert_eq!(c.sides, 10);
        assert_eq!(c.modifier, 1);
    }
    #[test]
    fn interactive_neg_modifier_works() {
        let mut c = Config::default();
        c.interact("d10-1".to_string());
        assert_eq!(c.sides, 10);
        assert_eq!(c.modifier, -1);
    }
    #[test]
    fn interactive_multi_die_works() {
        let mut c = Config::default();
        c.interact("3d6".to_string());
        assert_eq!(c.count, 3);
        assert_eq!(c.sides, 6);
    }
    #[test]
    fn interactive_advantage_works() {
        let mut c = Config::default();
        c.interact("d10 a".to_string());
        assert_eq!(c.sides, 10);
        assert!(c.advantage);
    }
    #[test]
    fn interactive_disadvantage_works() {
        let mut c = Config::default();
        c.interact("d10 d".to_string());
        assert_eq!(c.sides, 10);
        assert!(c.disadvantage);
    }
    #[test]
    fn interactive_mix_and_match_works() {
        let mut c = Config::default();
        c.interact("2d10+1 a".to_string());
        assert_eq!(c.sides, 10);
        assert_eq!(c.count, 2);
        assert_eq!(c.modifier, 1);
        assert!(c.advantage);
        c.interact("3d6-2 d".to_string());
        assert_eq!(c.count, 3);
        assert_eq!(c.sides, 6);
        assert_eq!(c.modifier, -2);
        assert!(c.disadvantage);
        c.interact("d6".to_string());
        assert_eq!(c.count, 1);
        assert_eq!(c.sides, 6);
    }
    #[test]
    fn bad_pos_modifier_1_arg_fails_gracefully() {
        let mut c = Config::default();
        c.interact("2d10+x".to_string());
        assert_eq!(c.modifier, 0);
    }
    #[test]
    fn bad_neg_modifier_1_arg_fails_gracefully() {
        let mut c = Config::default();
        c.interact("2d10-x".to_string());
        assert_eq!(c.modifier, 0);
    }
    #[test]
    fn bad_count_1_arg_fails_gracefully() {
        let mut c = Config::default();
        c.interact("xd10".to_string());
        assert_eq!(c.count, 1);
    }
    #[test]
    fn bad_sides_1_arg_fails_gracefully() {
        let mut c = Config::default();
        c.interact("2dx".to_string());
        assert_eq!(c.count, 2);
        assert_eq!(c.sides, 20);
        let mut c = Config::default();
        c.interact("xdx".to_string());
        assert_eq!(c.count, 1);
        assert_eq!(c.sides, 20);
    }
    #[test]
    fn bad_adv_dis_arg_fails_gracefully() {
        let mut c = Config::default();
        c.interact("2d10 b".to_string());
        assert!(!c.advantage);
        assert!(!c.disadvantage);
    }
    #[test]
    fn bad_pos_modifier_2_arg_fails_gracefully() {
        let mut c = Config::default();
        c.interact("2d10+x x".to_string());
        assert_eq!(c.count, 2);
        assert_eq!(c.sides, 10);
        assert_eq!(c.modifier, 0);
        assert!(!c.advantage);
        assert!(!c.disadvantage);
    }
    #[test]
    fn bad_neg_modifier_2_arg_fails_gracefully() {
        let mut c = Config::default();
        c.interact("2d10-x x".to_string());
        assert_eq!(c.count, 2);
        assert_eq!(c.sides, 10);
        assert_eq!(c.modifier, 0);
        assert!(!c.advantage);
        assert!(!c.disadvantage);
    }
    #[test]
    fn bad_count_and_sides_fails_gracefully() {
        let mut c = Config::default();
        c.interact("xdx+1 a".to_string());
        assert_eq!(c.count, 1);
        assert_eq!(c.sides, 20);
        assert!(c.advantage);
        assert_eq!(c.modifier, 1);
    }
    #[test]
    fn too_many_args_fails_to_default() {
        let mut c = Config::default();
        c.interact("a b c".to_string());
        assert_eq!(c.count, 1);
        assert_eq!(c.sides, 20);
        assert_eq!(c.modifier, 0);
    }
}
