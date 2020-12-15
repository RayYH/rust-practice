fn plus_one(x: i32) -> i32 {
    // variables are immutable only by default, we cannot use `x = x+1` statement
    // y has been modified by `mut` keyword, so y is mutable
    let mut y = x;
    y = y + 1;
    return y;
}

// use `mut` with constants is not allowed
// constants are always immutable.
const MAX_POINTS: u32 = 100_000;

#[cfg(test)]
mod tests {
    use crate::variables::{plus_one, MAX_POINTS};

    #[test]
    fn test_plus_one() {
        assert_eq!(6, plus_one(5));
        assert_eq!(7, plus_one(6));
        assert_eq!(-2, plus_one(-3));
    }

    #[test]
    fn test_max_points() {
        let x: u32 = MAX_POINTS - 1;
        assert_eq!(x, 99_999)
    }

    #[test]
    fn test_shadowing() {
        // By using let, we can perform a few transformations on a value but have the variable be
        // immutable after those transformations have been completed.
        let x = 5;
        assert_eq!(x, 5);
        let x = x + 1;
        assert_eq!(x, 6);
        let x = x * 2;
        assert_eq!(x, 12);
        // string type
        let spaces = "   ";
        // number type
        let spaces = spaces.len();
        assert_eq!(spaces, 3);
    }
}
