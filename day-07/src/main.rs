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

fn run_thrust_amplifiers_feedback(
    program: Vec<isize>,
    phase_settings: (isize, isize, isize, isize, isize),
) -> isize {
    let mut process_a = IntcodeProcess::from_vec(program.clone());
    process_a.add_input(phase_settings.0);
    let mut process_b = IntcodeProcess::from_vec(program.clone());
    process_b.add_input(phase_settings.1);
    let mut process_c = IntcodeProcess::from_vec(program.clone());
    process_c.add_input(phase_settings.2);
    let mut process_d = IntcodeProcess::from_vec(program.clone());
    process_d.add_input(phase_settings.3);
    let mut process_e = IntcodeProcess::from_vec(program.clone());
    process_e.add_input(phase_settings.4);

    let mut output_a = 0;
    let mut output_b = 0;
    let mut output_c = 0;
    let mut output_d = 0;
    let mut output_e = 0;

    loop {
        process_a.add_input(output_e);
        let result = process_a.run_to_output();
        match result {
            Ok(a) => {
                output_a = a;
            }
            Err(IntcodeError::CatchFire) => {
                break;
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        process_b.add_input(output_a);
        let result = process_b.run_to_output();
        match result {
            Ok(b) => {
                output_b = b;
            }
            Err(IntcodeError::CatchFire) => {
                panic!("process_b unexpectedly halted before process_a");
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        process_c.add_input(output_b);
        let result = process_c.run_to_output();
        match result {
            Ok(c) => {
                output_c = c;
            }
            Err(IntcodeError::CatchFire) => {
                panic!("process_c unexpectedly halted before process_a");
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        process_d.add_input(output_c);
        let result = process_d.run_to_output();
        match result {
            Ok(d) => {
                output_d = d;
            }
            Err(IntcodeError::CatchFire) => {
                panic!("process_d unexpectedly halted before process_a");
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        process_e.add_input(output_d);
        let result = process_e.run_to_output();
        match result {
            Ok(e) => {
                output_e = e;
            }
            Err(IntcodeError::CatchFire) => {
                panic!("process_e unexpectedly halted before process_a");
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }

    return output_e;
}

fn find_max_thrust_amplifier_feedback(
    program: Vec<isize>,
) -> (isize, (isize, isize, isize, isize, isize)) {
    let mut max = isize::min_value();
    let mut settings = (0, 0, 0, 0, 0);

    for a in 5..=9 {
        for b in 5..=9 {
            if b == a {
                continue;
            }
            for c in 5..=9 {
                if c == a || c == b {
                    continue;
                }
                for d in 5..=9 {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    for e in 5..=9 {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }
                        let s = (a, b, c, d, e);
                        let output = run_thrust_amplifiers_feedback(program.clone(), s);
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

    let (max, settings) = find_max_thrust_amplifier_feedback(program.clone());

    println!("max={} at {:?}", max, settings);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_a1() {
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
    fn test_example_a1_find() {
        let input = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];

        let (max, settings) = find_max_thrust_amplifier(input);

        assert_eq!(max, 43210);
        assert_eq!(settings, (4, 3, 2, 1, 0));
    }

    #[test]
    fn test_example_a2() {
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
    fn test_example_a2_find() {
        let input = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];

        let (max, settings) = find_max_thrust_amplifier(input);

        assert_eq!(max, 54321);
        assert_eq!(settings, (0, 1, 2, 3, 4));
    }

    #[test]
    fn test_example_b1() {
        let input = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];

        let output = run_thrust_amplifiers_feedback(input, (9, 8, 7, 6, 5));

        assert_eq!(output, 139629729);
    }

    #[test]
    fn test_example_b2() {
        let input = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];

        let output = run_thrust_amplifiers_feedback(input, (9, 7, 8, 5, 6));

        assert_eq!(output, 18216);
    }

    #[test]
    fn test_example_b2_find() {
        let input = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];

        let (max, settings) = find_max_thrust_amplifier_feedback(input);

        assert_eq!(max, 18216);
        assert_eq!(settings, (9, 7, 8, 5, 6));
    }
}
