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

    processor.store(1, 12).unwrap();
    processor.store(2, 2).unwrap();

    let result = processor.run();
    assert_eq!(result, Err(IntcodeError::CatchFire));
    println!("0: {}", processor.load(0).unwrap());

    'outer: for noun in 0..=99 {
        for verb in 0..=99 {
            let mut processor = IntcodeProcess::from_vec(memory.clone());
            processor.store(1, noun).unwrap();
            processor.store(2, verb).unwrap();
            let result = processor.run();
            assert_eq!(result, Err(IntcodeError::CatchFire));
            let output = processor.load(0).unwrap();
            if output == 19690720 {
                println!("noun={}, verb={}, answer={}", noun, verb, 100 * noun + verb);
                break 'outer;
            }
        }
    }
}
