// if Expressions
pub fn greater_than_five(number: i32) -> String {
    return if number > 5 {
        "number is greater than 5".parse().unwrap()
    } else {
        "number is not greater than 5".parse().unwrap()
    };
}

// Handling Multiple Conditions with else if
pub fn is_divisible(number: i32) -> String {
    return if number % 4 == 0 {
        "number is divisible by 4".parse().unwrap()
    } else if number % 3 == 0 {
        "number is divisible by 3".parse().unwrap()
    } else if number % 2 == 0 {
        "number is divisible by 2".parse().unwrap()
    } else {
        "number is not divisible by 4, 3, or 2".parse().unwrap()
    };
}

#[cfg(test)]
mod tests {
    use crate::control_flows::{greater_than_five, is_divisible};

    #[test]
    fn test_greater_than_five() {
        assert_eq!(greater_than_five(6), "number is greater than 5");
        assert_eq!(greater_than_five(4), "number is not greater than 5");
    }

    #[test]
    fn test_is_divisible() {
        assert_eq!(is_divisible(4), "number is divisible by 4");
        assert_eq!(is_divisible(3), "number is divisible by 3");
        assert_eq!(is_divisible(2), "number is divisible by 2");
        assert_eq!(is_divisible(1), "number is not divisible by 4, 3, or 2");
    }

    #[test]
    fn test_using_if_in_a_let_statement() {
        let condition = true;
        // if and else must have compatible types
        let number = if condition { 5 } else { 6 };
        assert_eq!(number, 5);
    }

    #[test]
    fn test_infinite_loop() {
        let mut i = 1;
        // infinite loop
        loop {
            assert!(i > 0);
            i += 1;
            if i == 100 {
                break;
            }
        }
    }

    #[test]
    fn test_return_values_from_loops() {
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        assert_eq!(20, result);
    }

    #[test]
    fn test_conditional_loops_with_while() {
        let mut number = 3;
        while number != 0 {
            number -= 1;
        }
        assert_eq!(number, 0);
    }

    #[test]
    fn test_loop_through_collection() {
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;
        let len = a.len();
        // this approach is error prone and slow
        while index < len {
            assert_eq!(10 * (1 + index), a[index]);
            index += 1;
        }
        // a better approach
        for element in a.iter() {
            assert!(*element > 0);
        }
        // use range
        for number in (1..4).rev() {
            assert!(number > 0);
        }
    }
}
