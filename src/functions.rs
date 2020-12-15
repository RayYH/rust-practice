// Rust code uses snake case as the conventional style for function and variable names.

fn my_echo(x: i32) -> i32 {
    return x;
}

fn addition(x: i32, y: i32) -> i32 {
    return x + y;
}

fn function_bodies() -> i32 {
    let x = 5;
    let y = {
        let x = 3;
        x + 1 // this is expression, so y = 4
    };
    return x + y;
}

#[cfg(test)]
mod tests {
    use crate::functions::{addition, function_bodies, my_echo};

    #[test]
    fn test_echo() {
        assert_eq!(my_echo(1), 1);
        assert_eq!(my_echo(2), 2);
    }

    #[test]
    fn test_addition() {
        assert_eq!(addition(3, 4), 7);
        assert_eq!(addition(12, 98), 110);
    }

    #[test]
    fn test_function_bodies() {
        assert_eq!(function_bodies(), 9);
    }
}
