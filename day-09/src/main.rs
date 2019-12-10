use intcode::{IntcodeError, IntcodeProcess};
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    let mut stdin = io::stdin();

    stdin.read_to_string(&mut input).unwrap();

    let program: Vec<isize> = input
        .trim()
        .split(",")
        .map(|s| s.parse::<isize>().unwrap())
        .collect();

    let mut process = IntcodeProcess::from_vec(program.clone());
    process.add_input(1);
    let result = process.run();

    assert_eq!(result, Err(IntcodeError::CatchFire));

    println!("{:?}", process.outputs());

    let mut process = IntcodeProcess::from_vec(program.clone());
    process.add_input(2);
    let result = process.run();

    assert_eq!(result, Err(IntcodeError::CatchFire));

    println!("{:?}", process.outputs());
}
