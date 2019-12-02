use std::io::{self, BufRead};
use std::str::FromStr;

fn simple_fuel_required(mass: usize) -> usize {
    if mass <= 8 {
        return 0;
    }

    ((mass as f64) / 3.0).floor() as usize - 2
}

#[derive(Debug)]
struct SpaceModule {
    mass: usize,
}

impl SpaceModule {
    fn new(mass: usize) -> SpaceModule {
        SpaceModule { mass }
    }

    fn fuel_required(&self) -> usize {
        simple_fuel_required(self.mass)
    }

    fn adjusted_fuel_required(&self) -> usize {
        let mut total = 0;
        let mut last = self.mass;
        loop {
            last = simple_fuel_required(last);
            if last == 0 {
                break;
            }
            total += last;
        }
        total
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
    let lines = stdin.lock().lines();

    let (fuel_required, adjusted_fuel_required) = lines
        .map(|line| line.unwrap().parse::<SpaceModule>().unwrap())
        .fold((0, 0), |(sum, adjusted_sum), module| {
            (
                sum + module.fuel_required(),
                adjusted_sum + module.adjusted_fuel_required(),
            )
        });

    println!("fuel required: {}", fuel_required);
    println!("adjusted fuel required: {}", adjusted_fuel_required);
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

    #[test]
    fn test_adjusted() {
        let sm = SpaceModule::new(12);
        assert_eq!(sm.adjusted_fuel_required(), 2);

        let sm = SpaceModule::new(14);
        assert_eq!(sm.adjusted_fuel_required(), 2);

        let sm = SpaceModule::new(1969);
        assert_eq!(sm.adjusted_fuel_required(), 966);

        let sm = SpaceModule::new(100756);
        assert_eq!(sm.adjusted_fuel_required(), 50346);
    }
}
