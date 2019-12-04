fn is_valid_number(num: usize) -> bool {
    let mut num = num;
    let mut last_numeral = num % 10;
    num = num / 10;

    let mut found_repeat = false;

    while num > 0 {
        let current_numeral = num % 10;

        if current_numeral > last_numeral {
            return false;
        }
        if current_numeral == last_numeral {
            found_repeat = true;
        }

        last_numeral = current_numeral;
        num = num / 10;
    }

    found_repeat
}

fn main() {
    let mut count = 0;

    for i in 372304..847060 {
        if is_valid_number(i) {
            count += 1;
        }
    }

    println!("{}", count);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(is_valid_number(111111), true);
        assert_eq!(is_valid_number(223450), false);
        assert_eq!(is_valid_number(123789), false);
    }
}
