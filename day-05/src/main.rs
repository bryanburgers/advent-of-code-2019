use intcode::{IntcodeError, IntcodeProcess};
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

    let mut processor = IntcodeProcess::from_vec(memory.clone());
    processor.add_input(1);
    let result = processor.run();
    assert_eq!(result, Err(IntcodeError::CatchFire));
    // Assert that everything but the last output is 0.
    let num_outputs = processor.outputs().len();
    assert!(processor
        .outputs()
        .iter()
        .take(num_outputs - 1)
        .all(|x| *x == 0));
    println!("{}", processor.outputs()[num_outputs - 1]);

    let mut processor = IntcodeProcess::from_vec(memory.clone());
    processor.add_input(5);
    let result = processor.run();
    assert_eq!(result, Err(IntcodeError::CatchFire));
    // Assert that everything but the last output is 0.
    let num_outputs = processor.outputs().len();
    assert!(processor
        .outputs()
        .iter()
        .take(num_outputs - 1)
        .all(|x| *x == 0));
    println!("{}", processor.outputs()[num_outputs - 1]);
}
