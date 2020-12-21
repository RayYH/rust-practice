// Rust is a statically typed language, which means that it
// must know the types of all variables at compile time.
#[cfg(test)]
mod tests {
    // signed integer types start with i
    // unsigned integer types start with u
    // Each signed variant can store numbers from -(2^(n-1)) to (2^(n-1))-1
    // Unsigned variants can store numbers from 0 to (2^n)-1
    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize
    #[test]
    fn test_scalar_integer_types() {
        let v_default = 32;
        let v_i8: i8 = 8;
        let v_i16: i16 = 16;
        let v_i32: i32 = 32;
        let v_i64: i64 = 64;
        let v_i128: i128 = 128;
        let v_isize: isize = 64;
        assert_eq!(32, v_default);
        assert_eq!(8, v_i8);
        assert_eq!(16, v_i16);
        assert_eq!(32, v_i32);
        assert_eq!(64, v_i64);
        assert_eq!(128, v_i128);
        assert_eq!(64, v_isize);
        let v_u8: u8 = 8;
        let v_u16: u16 = 16;
        let v_u32: u32 = 32;
        let v_u64: u64 = 64;
        let v_u128: u128 = 128;
        let v_usize: usize = 64;
        assert_eq!(8, v_u8);
        assert_eq!(16, v_u16);
        assert_eq!(32, v_u32);
        assert_eq!(64, v_u64);
        assert_eq!(128, v_u128);
        assert_eq!(64, v_usize);
        // number literals
        let decimal = 98_222;
        let hex = 0xff;
        let octal = 0o77;
        let binary = 0b1111_0000;
        let byte = b'A';
        assert_eq!(98222, decimal);
        assert_eq!(255, hex);
        assert_eq!(63, octal);
        assert_eq!(240, binary);
        assert_eq!(65, byte);
    }

    #[test]
    fn test_scalar_floating_point_numbers() {
        let x = 2.0; // f64
        let y: f32 = 3.0; // f32
        assert_eq!(2.0, x);
        assert_eq!(3.0, y);
    }

    #[test]
    fn test_scalar_numeric_operations() {
        // addition
        let sum = 5 + 10;
        assert_eq!(15, sum);
        // subtraction
        let difference = 95.5 - 4.3;
        assert_eq!(91.2, difference);
        // multiplication
        let product = 4 * 30;
        assert_eq!(120, product);
        // division
        let quotient = 56.7 / 32.2;
        assert!(quotient > 1.7);
        assert!(quotient < 1.8);
        // remainder
        let remainder = 43 % 5;
        assert_eq!(3, remainder);
    }

    #[test]
    fn test_scalar_boolean_types() {
        let t = true;
        assert!(t);
        let f: bool = false; // with explicit type annotation
        assert!(!f);
    }

    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value
    #[test]
    fn test_scalar_char_type() {
        let c = 'z';
        assert_eq!(c, 'z');
        let z = 'ℤ';
        assert_eq!(z, 'ℤ');
        let heart_eyed_cat = '😻';
        assert_eq!(heart_eyed_cat, '😻');
    }

    #[test]
    fn test_compound_tuple_type() {
        // auto infer
        let tup = (500, 6.4, 1);
        assert_eq!(tup.0, 500);
        // specify type
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        assert_eq!(tup.1, 6.4);
        // deconstruct
        let (_x, _y, z) = tup;
        assert_eq!(z, 1);
    }

    #[test]
    fn test_compound_array_type() {
        let a = [1, 2, 3, 4, 5];
        assert_eq!(a.len(), 5);
        assert_eq!(a[1], 2);
        let months = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];
        assert!(months.contains(&"February"));
        // specify type
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(a.len(), 5);
        let a = [3; 5];
        assert_eq!(a, [3, 3, 3, 3, 3])
    }
}
