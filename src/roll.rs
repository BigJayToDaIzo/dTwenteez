use crate::Config;
use rand::Rng;

#[derive(Debug)]
pub struct Roll {
    pub final_roll: u32,
    pub display: String,
}

impl Roll {
    pub fn new(c: &Config) -> Roll {
        // rng stuffs
        let mut rng = rand::rng();
        let final_roll = rng.random_range(1..=c.sides);
        // this will evolve heavily
        let roll_string = format!("[ {final_roll} ] = {final_roll}");
        Roll {
            final_roll,
            display: roll_string,
        }
        // build display
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use regex::Regex;

    #[test]
    fn roll_single_d20_no_modifiers() {
        let c = Config::default();
        let roll = Roll::new(&c);
        assert!((1..=20).contains(&roll.final_roll));
    }
    #[test]
    fn roll_single_d2_no_modifiers() {
        let mut c = Config::default();
        let mut args = ["", "-s", "2"].iter().map(|s| s.to_string());
        c.build(&mut args);
        let roll = Roll::new(&c);
        assert_eq!(c.sides, 2);
        let regex = Regex::new(r"\[ (?<summation_detail>\d) \] = (?<total>\d)").unwrap();
        let Some(cap) = regex.captures(&roll.display) else {
            panic!("display did not capture value");
        };
        assert_eq!(cap.len(), 3); // cap[0] is entire haystack
        assert!((1..=2).contains(&cap["summation_detail"].parse::<u32>().unwrap()));
        assert!((1..=2).contains(&cap["total"].parse::<u32>().unwrap()));
        assert_eq!(&cap["summation_detail"], &cap["total"]);
    }
    #[test]
    fn d20_no_modifiers_display() {
        // ex: [ 17 ] = 17
        let c = Config::default();
        let roll = Roll::new(&c);
        let regex = Regex::new(r"\[ (?<summation_detail>\d{1,2}) \] = (?<total>\d{1,2})").unwrap();
        let Some(cap) = regex.captures(&roll.display) else {
            panic!("display did not capture value");
        };
        assert_eq!(cap.len(), 3); // cap[0] is entire haystack
        assert!((1..=20).contains(&cap["summation_detail"].parse::<u32>().unwrap()));
        assert!((1..=20).contains(&cap["total"].parse::<u32>().unwrap()));
        assert_eq!(&cap["summation_detail"], &cap["total"]);
    }
}
