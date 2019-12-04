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

fn is_valid_number_2(num: usize) -> bool {
    let mut num = num;
    let mut last_numeral = num % 10;
    num = num / 10;

    let mut found_repeat = false;
    let mut current_repeat_count = 0;

    while num > 0 {
        let current_numeral = num % 10;

        if current_numeral > last_numeral {
            return false;
        }
        if current_numeral == last_numeral {
            if current_repeat_count == 0 {
                current_repeat_count = 2;
            } else {
                current_repeat_count += 1;
            }
        } else {
            if current_repeat_count == 2 {
                found_repeat = true;
            }
            current_repeat_count = 0;
        }

        last_numeral = current_numeral;
        num = num / 10;
    }

    found_repeat || current_repeat_count == 2
}

fn main() {
    let mut count = 0;

    for i in 372304..847060 {
        if is_valid_number(i) {
            count += 1;
        }
    }

    println!("{}", count);

    let mut count = 0;

    for i in 372304..847060 {
        if is_valid_number_2(i) {
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

    #[test]
    fn test_2() {
        assert_eq!(is_valid_number_2(111111), false);
        assert_eq!(is_valid_number_2(223450), false);
        assert_eq!(is_valid_number_2(123789), false);
        assert_eq!(is_valid_number_2(112233), true);
        assert_eq!(is_valid_number_2(123444), false);
        assert_eq!(is_valid_number_2(111122), true);

        assert_eq!(is_valid_number_2(111233), true);
        assert_eq!(is_valid_number_2(122223), false);
        assert_eq!(is_valid_number_2(122334), true);
        assert_eq!(is_valid_number_2(112345), true);
        assert_eq!(is_valid_number_2(112334), true);
        assert_eq!(is_valid_number_2(113334), true);
        assert_eq!(is_valid_number_2(133333), false);
        assert_eq!(is_valid_number_2(333335), false);
    }
}
