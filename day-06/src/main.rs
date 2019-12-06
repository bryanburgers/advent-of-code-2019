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

fn chain(key: &str, map: &HashMap<String, String>) -> Vec<String> {
    let mut vec = Vec::new();

    let mut key = key;
    vec.push(key.into());
    loop {
        if let Some(next) = map.get(key) {
            vec.push(next.into());
            key = next;
        } else {
            break;
        }
    }

    vec
}

fn common_ancestor(v1: &[String], v2: &[String]) -> Option<String> {
    let mut result = None;

    for i in 0..v1.len() {
        if v1[i] == v2[i] {
            result = Some(v1[i].clone());
        } else {
            break;
        }
    }

    result
}

fn jumps_between(k1: &str, k2: &str, map: &HashMap<String, String>) -> usize {
    let mut k1_chain = chain(k1, map);
    k1_chain.reverse();
    let mut k2_chain = chain(k2, map);
    k2_chain.reverse();

    let common_ancestor =
        common_ancestor(&k1_chain, &k2_chain).expect("k1 and k2 should have a common ancestor");

    let common_ancestor_len = path_size(&common_ancestor, map);
    let k1_len = k1_chain.len();
    let k2_len = k2_chain.len();

    let distance_from_k1_to_ancestor = k1_len - common_ancestor_len - 1;
    let distance_from_k2_to_ancestor = k2_len - common_ancestor_len - 1;

    distance_from_k1_to_ancestor - 1 + distance_from_k2_to_ancestor - 1
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let listings = lines.map(|line| line.unwrap().parse::<OrbitListing>().unwrap());

    let map = build_map(listings);

    let checksum = checksum(&map);

    println!("checksum={}", checksum);

    let jumps_between = jumps_between("YOU", "SAN", &map);

    println!("jumps_between={}", jumps_between);
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

    #[test]
    fn test_chain() {
        let orbits = &[
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ];

        let orbits = orbits
            .into_iter()
            .map(|listing| listing.parse::<OrbitListing>().unwrap());

        let map = build_map(orbits);

        assert_eq!(chain("J", &map), vec!["J", "E", "D", "C", "B", "COM"]);
    }

    #[test]
    fn test_common_ancestor() {
        let v1 = vec![
            "0".into(),
            "1".into(),
            "2".into(),
            "3".into(),
            "4".into(),
            "5".into(),
            "6".into(),
            "7".into(),
            "8".into(),
            "9".into(),
            "10".into(),
        ];
        let v2 = vec![
            "0".into(),
            "1".into(),
            "2".into(),
            "3".into(),
            "4".into(),
            "11".into(),
            "12".into(),
            "13".into(),
        ];

        assert_eq!(common_ancestor(&v1[..], &v2[..]), Some("4".into()));
    }

    #[test]
    fn test_jumps_between() {
        let orbits = &[
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ];

        let orbits = orbits
            .into_iter()
            .map(|listing| listing.parse::<OrbitListing>().unwrap());

        let map = build_map(orbits);

        assert_eq!(jumps_between("YOU", "SAN", &map), 4);
    }
}
