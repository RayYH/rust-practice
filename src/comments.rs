// Regular comments which are ignored by the compiler: `//` and `/* */`.
// Doc comments which are parsed into HTML library documentation: /// and //!

fn simple_value() -> i32 {
    let x = 5 + /* 90 + */ 5;
    return x;
}

#[cfg(test)]
mod tests {
    use crate::comments::simple_value;

    #[test]
    fn test_simple_value() {
        assert_eq!(10, simple_value());
    }
}
