mod intcode;
use intcode::{Intcode, IntcodeError};
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    let mut stdin = io::stdin();

    stdin.read_to_string(&mut input).unwrap();

    let memory: Vec<isize> = input
        .trim()
        .split(",")
        .map(|s| s.parse::<isize>().unwrap())
        .collect();

    let mut processor = Intcode::from_vec(memory);

    processor.store(1, 12).unwrap();
    processor.store(2, 2).unwrap();

    let result = processor.run();
    assert_eq!(result, Err(IntcodeError::CatchFire));
    println!("0: {}", processor.load(0).unwrap());
}
