// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

fn takes_ownership(some_string: String) -> String {
    // some_string comes into scope
    return some_string.parse().unwrap();
} // some_string goes out of scope and `drop` is called and the backing memory is freed

fn makes_copy(some_integer: i32) -> i32 {
    // some_integer comes into scope
    return some_integer;
} // some_integer goes out of scope and nothing special happens

// gives_ownership will move its return value into the function that calls it
fn gives_ownership() -> String {
    // some_string comes into scope
    let some_string = String::from("hello");
    // some_string is returned and moves out to the calling function
    some_string
}

// takes_and_gives_back will take a String and return one
#[allow(dead_code)]
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string
    // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

// take &String rather than String
fn calculate_length_via_reference(s: &String) -> usize {
    // s is a reference to a String
    // When functions have references as parameters instead of the actual values, we won’t need to
    // return the values in order to give back ownership, because we never had ownership.
    s.len()
} // s goes out of scope, because it does not have ownership of what it refers to, nothing happens

fn change(some_string: &mut String) {
    some_string.push_str("!");
}

// &str allows us to use the same function on both &String values and &str values.
fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    // enumerate method returns a tuple, we can use patterns to destructure that tuple
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

#[cfg(test)]
mod tests {
    use crate::ownership::{
        calculate_length, calculate_length_via_reference, change, first_world, gives_ownership,
        makes_copy, takes_and_gives_back, takes_ownership,
    };

    #[test]
    fn test_variable_scope() {
        // when s comes into scope, it is valid.
        {
            // s is not valid here, it’s not yet declared
            let s = "hello"; // s is valid from this point forward
            assert_eq!(s, "hello"); // do stuff with s
        } // this scope is now over, and s is no longer valid
          // it remains valid until it goes out of scope.
    }

    #[test]
    fn test_string_type() {
        // the double colon (::) is an operator that allows us to namespace this particular from
        // function under the String type rather than using some sort of name like string_from.
        let s = String::from("hello");
        assert_eq!("hello", s);
        let mut s = String::from("hello");
        s.push_str(", world!");
        assert_eq!(s, "hello, world!");
    }

    // Without a GC, it’s our responsibility to identify when memory is no longer being
    // used and call code to explicitly return it.
    // Rust will never automatically create “deep” copies of your data. Therefore, any
    // automatic copying can be assumed to be inexpensive in terms of runtime performance.

    #[test]
    fn test_string_clone() {
        let s1 = String::from("hello");
        // deep copy is expensive
        let s2 = s1.clone();
        assert_eq!(s1, s2);
    }

    // Rust has a special annotation called the Copy trait that we can place on types like
    // integers that are stored on the stack.
    // If a type has the Copy trait, an older variable is still usable after assignment.
    #[test]
    fn test_ownerships_and_functions() {
        // s comes into scope
        let s = String::from("hello");
        // s's value moves into the function  and so is no longer valid here
        assert_eq!(takes_ownership(s), "hello");
        // if we access s here, an error will be reported

        // x comes into scope
        let x = 5;
        // x would move into the function, but i32 is Copy, so it’s okay to still use x afterward
        assert_eq!(5, makes_copy(x));
        assert_eq!(x, 5);
    }

    // When a variable that includes data on the heap goes out of scope, the value will be cleaned
    // up by drop unless the data has been moved to be owned by another variable.
    #[test]
    fn test_return_values_and_scope() {
        // gives_ownership moves its return value into s1
        let s1 = gives_ownership();
        assert_eq!(s1, "hello");
        // s2 comes into scope
        let s2 = String::from("hello");
        assert_eq!(s2, "hello");
        // s2 is moved into takes_and_gives_back, which also moves its return value into s3
        let s3 = takes_and_gives_back(s2);
        assert_eq!(s3, "hello");
    }
    // s3 goes out of scope and is dropped.
    // s2 goes out of scope but was moved (takes_and_gives_back), so nothing happens.
    // s1 goes out of scope and is dropped.

    #[test]
    fn test_calculate_length() {
        // It’s quite annoying that anything we pass in also needs to be passed back if we want to
        // use it again, in addition to any data resulting from the body of the function that we
        // might want to return as well. It’s possible to return multiple values using a tuple.
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        assert_eq!(s2, "hello");
        assert_eq!(len, 5);
    }

    // References allow you to refer to some value without taking ownership of it.
    #[test]
    fn test_calculate_length_via_reference() {
        let s1 = String::from("hello");
        // &s1: create a reference that refers to the value of s1 but does not own it
        let len = calculate_length_via_reference(&s1);
        assert_eq!(s1, "hello");
        assert_eq!(len, 5);
    }

    #[test]
    fn test_change_string() {
        let mut s = String::from("hello");
        {
            let r1 = &mut s;
            // You can have only one mutable reference to a particular piece of data in a
            // particular scope. In this case, we cannot use `let r2 = &mut s;` statement here.
            change(r1);
            assert_eq!(s, "hello!");
        } // r1 goes out of scope here, so we can make a new reference with no problems.
        let r2 = &mut s;
        change(r2);
        assert_eq!(s, "hello!!")
    }

    #[test]
    fn test_reference_scope() {
        // A reference’s scope starts from where it is introduced and continues through the
        // last time that reference is used.
        let mut s = String::from("hello");
        // At any given time, you can have either one mutable reference or any
        // number of immutable references.
        let r1 = &s; // no problem
        let r2 = &s; // no problem
        assert_eq!(r1, r2);
        // r1 and r2 are no longer used after this point
        let r3 = &mut s; // no problem
        assert_eq!(r3, "hello");
        // if we try to access r1 here, this program will not be compiled successfully
    }

    #[test]
    fn test_string_slices() {
        let s = String::from("hello world");
        let hello = &s[0..5];
        let world = &s[6..11];
        assert_eq!(hello, "hello");
        assert_eq!(world, "world");
        // drop the value before two periods
        assert_eq!(&s[0..2], &s[..2]);
        let len = s.len();
        // drop trailing number
        assert_eq!(&s[3..len], &s[3..]);
        // drop both values
        assert_eq!(&s[0..len], &s[..]);
    }

    #[test]
    fn test_first_world() {
        let my_string = String::from("hello world");
        assert_eq!(first_world(&my_string[..]), "hello");
        let my_string_literal = "hello world";
        assert_eq!(first_world(&my_string_literal[..]), "hello");
        assert_eq!(first_world(my_string_literal), "hello");
    }
}
