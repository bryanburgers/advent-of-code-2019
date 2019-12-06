use std::collections::HashMap;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct OrbitListing {
    orbitee: String,
    orbiter: String,
}

impl FromStr for OrbitListing {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iter = input.split(")");
        let orbitee = iter.next().ok_or(())?.into();
        let orbiter = iter.next().ok_or(())?.into();

        if let Some(_) = iter.next() {
            return Err(());
        }

        Ok(OrbitListing { orbitee, orbiter })
    }
}

fn build_map(listings: impl Iterator<Item = OrbitListing>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for listing in listings {
        map.insert(listing.orbiter, listing.orbitee);
    }

    map
}

fn checksum(map: &HashMap<String, String>) -> usize {
    let mut checksum = 0;

    for key in map.keys() {
        checksum += path_size(key, map);
    }

    checksum
}

fn path_size(key: &str, map: &HashMap<String, String>) -> usize {
    if let Some(value) = map.get(key) {
        1 + path_size(value, map)
    } else {
        0
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let listings = lines.map(|line| line.unwrap().parse::<OrbitListing>().unwrap());

    let map = build_map(listings);

    let checksum = checksum(&map);

    println!("checksum={}", checksum);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let result = "COM)A".parse::<OrbitListing>();

        assert_eq!(
            result,
            Ok(OrbitListing {
                orbitee: "COM".into(),
                orbiter: "A".into()
            })
        );
    }

    #[test]
    fn test_checksum() {
        let orbits = &[
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ];

        let orbits = orbits
            .into_iter()
            .map(|listing| listing.parse::<OrbitListing>().unwrap());

        let map = build_map(orbits);

        assert_eq!(checksum(&map), 42);
    }
}
