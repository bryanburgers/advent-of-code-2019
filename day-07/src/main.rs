use intcode::{IntcodeError, IntcodeProcess};
use std::io::{self, Read};

fn run_thrust_amplifier(program: Vec<isize>, phase_setting: isize, input_signal: isize) -> isize {
    let mut process = IntcodeProcess::from_vec(program);
    process.add_input(phase_setting);
    process.add_input(input_signal);
    let result = process.run();
    assert_eq!(result, Err(IntcodeError::CatchFire));

    assert!(process.outputs().len() >= 1);

    process.outputs()[0]
}

fn find_max_thrust_amplifier(program: Vec<isize>) -> (isize, (isize, isize, isize, isize, isize)) {
    let mut max = isize::min_value();
    let mut settings = (0, 0, 0, 0, 0);

    for a in 0..=4 {
        let output = run_thrust_amplifier(program.clone(), a, 0);
        for b in 0..=4 {
            if b == a {
                continue;
            }
            let output = run_thrust_amplifier(program.clone(), b, output);
            for c in 0..=4 {
                if c == a || c == b {
                    continue;
                }
                let output = run_thrust_amplifier(program.clone(), c, output);
                for d in 0..=4 {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    let output = run_thrust_amplifier(program.clone(), d, output);
                    for e in 0..=4 {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }
                        let output = run_thrust_amplifier(program.clone(), e, output);
                        if output > max {
                            max = output;
                            settings = (a, b, c, d, e);
                        }
                    }
                }
            }
        }
    }

    (max, settings)
}

fn main() {
    let mut input = String::new();
    let mut stdin = io::stdin();

    stdin.read_to_string(&mut input).unwrap();

    let program: Vec<isize> = input
        .trim()
        .split(",")
        .map(|s| s.parse::<isize>().unwrap())
        .collect();

    let (max, settings) = find_max_thrust_amplifier(program.clone());

    println!("max={} at {:?}", max, settings);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];

        let output = run_thrust_amplifier(input.clone(), 4, 0);
        let output = run_thrust_amplifier(input.clone(), 3, output);
        let output = run_thrust_amplifier(input.clone(), 2, output);
        let output = run_thrust_amplifier(input.clone(), 1, output);
        let output = run_thrust_amplifier(input.clone(), 0, output);

        assert_eq!(output, 43210);
    }

    #[test]
    fn test_example_1_find() {
        let input = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];

        let (max, settings) = find_max_thrust_amplifier(input);

        assert_eq!(max, 43210);
        assert_eq!(settings, (4, 3, 2, 1, 0));
    }

    #[test]
    fn test_example_2() {
        let input = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];

        let output = run_thrust_amplifier(input.clone(), 0, 0);
        let output = run_thrust_amplifier(input.clone(), 1, output);
        let output = run_thrust_amplifier(input.clone(), 2, output);
        let output = run_thrust_amplifier(input.clone(), 3, output);
        let output = run_thrust_amplifier(input.clone(), 4, output);

        assert_eq!(output, 54321);
    }

    #[test]
    fn test_example_2_find() {
        let input = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];

        let (max, settings) = find_max_thrust_amplifier(input);

        assert_eq!(max, 54321);
        assert_eq!(settings, (0, 1, 2, 3, 4));
    }
}
