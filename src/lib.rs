use rand::Rng;

#[derive(Debug)]
pub struct Roll {
    pub final_roll: u32,
    pub display: String,
}

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
                        }
                    }
                }
                "-c" => {
                    if let Some(arg) = args.next() {
                        if let Ok(count) = arg.parse::<u32>() {
                            self.count = count;
                        }
                    }
                }
                "-m" => {
                    if let Some(arg) = args.next() {
                        if let Ok(modifier) = arg.parse::<i32>() {
                            self.modifier = modifier;
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
            println!(
                "usage: d20 [-h | --help | -a | -d | -c N | -s N | -m +|-N]
** example d20 -a -c 1 -s 20 -m +9
** Astarion rolls with advantage, one d20 with a +9 modifier"
            );
        }
    }
    pub fn roll(&self) -> Roll {
        // rng stuffs
        let mut rng = rand::rng();
        let final_roll = rng.random_range(1..=self.sides);
        Roll {
            final_roll,
            display: String::new(),
        }
        // build display
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() {
        let c = Config::default();
        assert_eq!(c.sides, 20);
        assert_eq!(c.count, 1);
        assert_eq!(c.modifier, 0);
    }
    #[test]
    fn build_works() {
        let mut c = Config::default();
        let mut args = ["", "-h", "-d", "-m", "-2"].iter().map(|s| s.to_string());
        c.build(&mut args);
        assert!(c.h_opt);
        assert!(c.disadvantage);
        assert_eq!(c.modifier, -2);
    }
    #[test]
    fn roll_single_d20_no_modifiers() {
        let c = Config::default();
        let roll = c.roll();
        assert!((1..=20).contains(&roll.final_roll));
    }
}
