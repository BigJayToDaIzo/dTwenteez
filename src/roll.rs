use crate::Config;
use rand::Rng;

#[derive(Debug)]
pub struct Roll {
    pub final_roll: u32,
    pub roll_log: String,
}

impl Roll {
    pub fn new(c: &Config) -> Roll {
        let mut rng = rand::rng();
        // this will evolve heavily
        let mut rt_roll = 0;
        let num_dice = c.count;
        let num_sides = c.sides;
        let mut roll_log = format!("{num_dice}d{num_sides} [ ");

        if c.advantage {
            for _ in 0..c.count {
                let roll1 = rng.random_range(1..=c.sides);
                let roll2 = rng.random_range(1..=c.sides);
                let max = roll1.max(roll2) as i32;
                let string_appendage = format!("[{roll1},{roll2}] max: {max} ");
                rt_roll += max;
                roll_log.push_str(&string_appendage);
            }
        } else if c.disadvantage {
            for _ in 0..c.count {
                let roll1 = rng.random_range(1..=c.sides);
                let roll2 = rng.random_range(1..=c.sides);
                let min = roll1.min(roll2) as i32;
                let string_appendage = format!("[{roll1},{roll2}] min: {min} ");
                rt_roll += min;
                roll_log.push_str(&string_appendage);
            }
        } else {
            for _ in 0..c.count {
                let roll = rng.random_range(1..=c.sides) as i32;
                rt_roll += roll;
                let string_appendage = format!("{roll} ");
                roll_log.push_str(&string_appendage);
            }
        }
        match c.modifier {
            i32::MIN..0 => {
                let modifier = c.modifier;
                // we MUST ensure roll never goes below 1
                if modifier + rt_roll < 1 {
                    rt_roll = 1;
                } else {
                    rt_roll += modifier;
                }
                let final_string_appendage = format!("] mod: {modifier}");
                roll_log.push_str(&final_string_appendage);
            }
            0 => {
                roll_log.push(']');
            }
            1..=i32::MAX => {
                let modifier = c.modifier;
                rt_roll += modifier;
                let final_string_appendage = format!("] mod: +{modifier}");
                roll_log.push_str(&final_string_appendage);
            }
        }
        Roll {
            final_roll: rt_roll as u32,
            roll_log,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use regex::Regex;

    fn build_config(args: Vec<&str>) -> Config {
        let mut c = Config::default();
        let mut args = args.iter().map(|s| s.to_string());
        c.build(&mut args);
        c
    }

    #[test]
    fn roll_d2_no_modifiers() {
        let c = build_config(vec!["", "-s", "2"]);
        let roll = Roll::new(&c);
        assert!((1..=c.sides).contains(&roll.final_roll));
    }
    #[test]
    fn roll_d20_no_modifiers() {
        let c = Config::default();
        let roll = Roll::new(&c);
        assert!((1..=c.sides).contains(&roll.final_roll));
    }
    #[test]
    fn d2_no_modifiers_log() {
        // ex: [ 2 ] = 2
        let c = build_config(vec!["", "-s", "2"]);
        let roll = Roll::new(&c);
        let regex = Regex::new(r"\d{1,2}?d\d{1,2} \[ (?<summation_detail>\d) \]").unwrap();
        let Some(cap) = regex.captures(&roll.roll_log) else {
            panic!("display did not capture value");
        };
        assert_eq!(c.sides, 2);
        assert_eq!(cap.len(), 2); // cap[0] is entire haystack
        assert!((1..=c.sides).contains(&cap["summation_detail"].parse::<u32>().unwrap()));
    }
    #[test]
    fn d20_no_modifiers_log() {
        let c = Config::default();
        let roll = Roll::new(&c);
        let regex = Regex::new(r"\d{1,2}?d\d{1,2} \[ (?<summation_detail>\d{1,2}) \]").unwrap();
        let Some(cap) = regex.captures(&roll.roll_log) else {
            panic!("display did not capture values");
        };
        assert_eq!(cap.len(), 2); // cap[0] is entire haystack
        assert!((1..=c.sides).contains(&cap["summation_detail"].parse::<u32>().unwrap()));
    }
    #[test]
    fn d20_pos_modifier_log() {
        // [ 10 ] mod: +2 = 12
        let c = build_config(vec!["", "-m", "2"]);
        let roll = Roll::new(&c);
        let regex =
            Regex::new(r"\d{1,2}?d\d{1,2} \[ (?<roll>\d{1,2}) \] mod: (?<modifier>\+\d{1,2})")
                .unwrap();
        let Some(cap) = regex.captures(&roll.roll_log) else {
            panic!("display didn't capture values")
        };
        assert_eq!(cap.len(), 3);
        assert!((1..=c.sides).contains(&cap["roll"].parse::<u32>().unwrap()));
        assert_eq!(cap["modifier"].parse::<u32>().unwrap(), 2);
    }
    #[test]
    fn d20_neg_modifier_log() {
        // [ 10 ] mod: -2 = 8
        let c = build_config(vec!["", "-m", "-2"]);
        let roll = Roll::new(&c);
        let regex =
            Regex::new(r"\d{1,2}?d\d{1,2} \[ (?<roll>\d{1,2}) \] mod: (?<modifier>\-\d{1,2})")
                .unwrap();
        let Some(cap) = regex.captures(&roll.roll_log) else {
            panic!("display didn't capture values")
        };
        assert_eq!(cap.len(), 3);
        assert!((1..=c.sides).contains(&cap["roll"].parse::<u32>().unwrap()));
        assert_eq!(cap["modifier"].parse::<i32>().unwrap(), -2);
    }
    #[test]
    fn neg_modifier_does_not_lower_roll_below_one() {
        // [ 1 ] mod: -1 = 1
        let c = build_config(vec!["", "-s", "2", "-m", "-3"]);
        let roll = Roll::new(&c);
        assert_eq!(roll.final_roll, 1);
    }
    #[test]
    fn advantage_log_works() {
        // [ [1,2] max: 2 ] = 2
        let c = build_config(vec!["", "-a"]);
        let roll = Roll::new(&c);
        let regex = Regex::new(
            r"\d{1,2}?d\d{1,2} \[ \[(?<roll1>\d{1,2}),(?<roll2>\d{1,2})\] max: (?<max>\d{1,2}) \]",
        )
        .unwrap();
        let Some(cap) = regex.captures(&roll.roll_log) else {
            panic!("display didn't capture values")
        };
        assert_eq!(cap.len(), 4);
        let max = cap["max"].parse::<u32>().unwrap();
        let roll1 = cap["roll1"].parse::<u32>().unwrap();
        let roll2 = cap["roll2"].parse::<u32>().unwrap();
        assert!(max >= roll1 && max >= roll2);
    }
    #[test]
    fn disadvantage_log_works() {
        // [ [1,2] max: 2 ] = 2
        let c = build_config(vec!["", "-d"]);
        let roll = Roll::new(&c);
        let regex = Regex::new(
            r"\d{1,2}?d\d{1,2} \[ \[(?<roll1>\d{1,2}),(?<roll2>\d{1,2})\] min: (?<min>\d{1,2}) \]",
        )
        .unwrap();
        let Some(cap) = regex.captures(&roll.roll_log) else {
            panic!("display didn't capture values")
        };
        assert_eq!(cap.len(), 4);
        let min = cap["min"].parse::<u32>().unwrap();
        let roll1 = cap["roll1"].parse::<u32>().unwrap();
        let roll2 = cap["roll2"].parse::<u32>().unwrap();
        assert!(min <= roll1 || min <= roll2);
    }
}
