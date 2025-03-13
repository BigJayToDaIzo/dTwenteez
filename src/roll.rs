use crate::Config;
use rand::Rng;

#[derive(Debug)]
pub struct Roll {
    pub final_roll: u32,
    pub display: String,
}

impl Roll {
    pub fn new(c: &Config) -> Roll {
        let mut rng = rand::rng();
        // this will evolve heavily
        // build display
        let mut display: String = String::from("[ ");
        let mut rt_roll = 0;
        if c.advantage {
            for _ in 0..c.count {
                let roll1 = rng.random_range(1..=c.sides);
                let roll2 = rng.random_range(1..=c.sides);
                let max = roll1.max(roll2) as i32;
                let string_appendage = format!("[{roll1},{roll2}] max: {max} ");
                rt_roll += max;
                display.push_str(&string_appendage);
            }
        } else if c.disadvantage {
            for _ in 0..c.count {
                let roll1 = rng.random_range(1..=c.sides);
                let roll2 = rng.random_range(1..=c.sides);
                let min = roll1.min(roll2) as i32;
                let string_appendage = format!("[{roll1},{roll2}] min: {min} ");
                rt_roll += min;
                display.push_str(&string_appendage);
            }
        } else {
            for _ in 0..c.count {
                let roll = rng.random_range(1..=c.sides) as i32;
                rt_roll += roll;
                let string_appendage = format!("{roll} ");
                display.push_str(&string_appendage);
            }
        }
        // has modifier?
        match c.modifier {
            i32::MIN..0 => {
                let modifier = c.modifier;
                // we MUST ensure roll never goes below 1
                if modifier + rt_roll < 1 {
                    rt_roll = 1;
                } else {
                    rt_roll += modifier;
                }
                let final_string_appendage = format!("] mod: {modifier} = {rt_roll}");
                display.push_str(&final_string_appendage);
            }
            0 => {
                let final_string_appendage = format!("] = {rt_roll}");
                display.push_str(&final_string_appendage);
            }
            1..=i32::MAX => {
                let modifier = c.modifier;
                rt_roll += modifier;
                let final_string_appendage = format!("] mod: +{modifier} = {rt_roll}");
                display.push_str(&final_string_appendage);
            }
        }
        Roll {
            final_roll: rt_roll as u32,
            display,
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
    fn d2_no_modifiers_display() {
        // ex: [ 2 ] = 2
        let c = build_config(vec!["", "-s", "2"]);
        let roll = Roll::new(&c);
        let regex = Regex::new(r"\[ (?<summation_detail>\d) \] = (?<total>\d)").unwrap();
        let Some(cap) = regex.captures(&roll.display) else {
            panic!("display did not capture value");
        };
        assert_eq!(c.sides, 2);
        assert_eq!(cap.len(), 3); // cap[0] is entire haystack
        assert!((1..=c.sides).contains(&cap["summation_detail"].parse::<u32>().unwrap()));
        assert!((1..=c.sides).contains(&cap["total"].parse::<u32>().unwrap()));
        assert_eq!(&cap["summation_detail"], &cap["total"]);
    }
    #[test]
    fn d20_no_modifiers_display() {
        let c = Config::default();
        let roll = Roll::new(&c);
        let regex = Regex::new(r"\[ (?<summation_detail>\d{1,2}) \] = (?<total>\d{1,2})").unwrap();
        let Some(cap) = regex.captures(&roll.display) else {
            panic!("display did not capture values");
        };
        assert_eq!(cap.len(), 3); // cap[0] is entire haystack
        assert!((1..=c.sides).contains(&cap["summation_detail"].parse::<u32>().unwrap()));
        assert!((1..=c.sides).contains(&cap["total"].parse::<u32>().unwrap()));
        assert_eq!(&cap["summation_detail"], &cap["total"]);
    }
    #[test]
    fn d20_pos_modifier_display() {
        // [ 10 ] mod: +2 = 12
        let c = build_config(vec!["", "-m", "2"]);
        let roll = Roll::new(&c);
        let regex =
            Regex::new(r"\[ (?<roll>\d{1,2}) \] mod: (?<modifier>\+\d{1,2}) = (?<total>\d{1,2})")
                .unwrap();
        let Some(cap) = regex.captures(&roll.display) else {
            panic!("display didn't capture values")
        };
        assert_eq!(cap.len(), 4);
        assert!((1..=c.sides).contains(&cap["roll"].parse::<u32>().unwrap()));
        assert_eq!(cap["modifier"].parse::<u32>().unwrap(), 2);
        assert!((1..=(c.sides + 2)).contains(&cap["total"].parse::<u32>().unwrap()));
    }
    #[test]
    fn d20_neg_modifier_display() {
        // [ 10 ] mod: -2 = 8
        let c = build_config(vec!["", "-m", "-2"]);
        let roll = Roll::new(&c);
        let regex =
            Regex::new(r"\[ (?<roll>\d{1,2}) \] mod: (?<modifier>\-\d{1,2}) = (?<total>\d{1,2})")
                .unwrap();
        let Some(cap) = regex.captures(&roll.display) else {
            panic!("display didn't capture values")
        };
        assert_eq!(cap.len(), 4);
        assert!((1..=c.sides).contains(&cap["roll"].parse::<u32>().unwrap()));
        assert_eq!(cap["modifier"].parse::<i32>().unwrap(), -2);
        assert!((1..=(c.sides - 2)).contains(&cap["total"].parse::<u32>().unwrap()));
    }
    #[test]
    fn neg_modifier_does_not_lower_roll_below_one() {
        // [ 1 ] mod: -1 = 1
        let c = build_config(vec!["", "-s", "2", "-m", "-2"]);
        let roll = Roll::new(&c);
        assert_eq!(roll.final_roll, 1);
    }
    #[test]
    fn advantage_display_works() {
        // [ [1,2] max: 2 ] = 2
        let c = build_config(vec!["", "-a"]);
        let roll = Roll::new(&c);
        let regex = Regex::new(
            r"\[ \[(?<roll1>\d{1,2}),(?<roll2>\d{1,2})\] max: (?<max>\d{1,2}) \] = (?<final>\d{1,2})",
        )
        .unwrap();
        let Some(cap) = regex.captures(&roll.display) else {
            panic!("display didn't capture values")
        };
        assert_eq!(cap.len(), 5);
        assert_eq!(cap["max"], cap["final"]);
    }
    #[test]
    fn disadvantage_display_works() {
        // [ [1,2] max: 2 ] = 2
        let c = build_config(vec!["", "-d"]);
        let roll = Roll::new(&c);
        let regex = Regex::new(
            r"\[ \[(?<roll1>\d{1,2}),(?<roll2>\d{1,2})\] min: (?<min>\d{1,2}) \] = (?<final>\d{1,2})",
        )
        .unwrap();
        let Some(cap) = regex.captures(&roll.display) else {
            panic!("display didn't capture values")
        };
        assert_eq!(cap.len(), 5);
        assert_eq!(cap["min"], cap["final"]);
    }
}
