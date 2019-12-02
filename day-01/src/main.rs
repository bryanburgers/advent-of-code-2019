use std::io::{self, BufRead, Read};
use std::str::FromStr;

#[derive(Debug)]
struct SpaceModule {
    mass: usize,
}

impl SpaceModule {
    fn new(mass: usize) -> SpaceModule {
        SpaceModule { mass }
    }

    fn fuel_required(&self) -> usize {
        ((self.mass as f64) / 3.0).floor() as usize - 2
    }
}

impl FromStr for SpaceModule {
    type Err = std::num::ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mass = input.parse()?;

        Ok(SpaceModule { mass })
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let fuel_required = lines
        .map(|line| line.unwrap().parse::<SpaceModule>().unwrap())
        .fold(0, |sum, module| sum + module.fuel_required());

    println!("fuel required: {}", fuel_required);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let sm = SpaceModule::new(12);
        assert_eq!(sm.fuel_required(), 2);

        let sm = SpaceModule::new(14);
        assert_eq!(sm.fuel_required(), 2);

        let sm = SpaceModule::new(1969);
        assert_eq!(sm.fuel_required(), 654);

        let sm = SpaceModule::new(100756);
        assert_eq!(sm.fuel_required(), 33583);
    }
}
