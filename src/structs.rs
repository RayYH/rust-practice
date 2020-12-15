// define a struct
struct User {
    // fields
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// we can define another impl block
impl Rectangle {
    // Associated Functions
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::{area, build_user, Color, Point, Rectangle, User};

    #[test]
    fn test_user_instance() {
        // Rust doesn’t allow us to mark only certain fields as mutable.
        let mut ray = User {
            username: String::from("Ray"),
            email: "".to_string(),
            sign_in_count: 1,
            active: true,
        };
        // since `ray` is mutable, we can change the email
        ray.email = String::from("rayyounghong@gmail.com");
        assert_eq!(ray.username, "Ray");
        assert_eq!(ray.email, "rayyounghong@gmail.com");
        assert_eq!(ray.sign_in_count, 1);
        assert!(ray.active);
    }

    #[test]
    fn test_build_user() {
        let ray = build_user("rayyounghong@gmail.com".to_string(), "Ray".to_string());
        assert_eq!(ray.username, "Ray");
        assert_eq!(ray.email, "rayyounghong@gmail.com");
        assert_eq!(ray.sign_in_count, 1);
        assert!(ray.active);
    }

    #[test]
    fn test_update_struct() {
        let ray = build_user("rayyounghong@gmail.com".to_string(), "Ray".to_string());
        assert_eq!(ray.username, "Ray");
        assert_eq!(ray.email, "rayyounghong@gmail.com");
        assert_eq!(ray.sign_in_count, 1);
        assert!(ray.active);
        let someone: User = User {
            email: "someone@some.com".to_string(),
            username: "someone".to_string(),
            ..ray
        };
        assert_eq!(someone.username, "someone");
        assert_eq!(someone.email, "someone@some.com");
        assert_eq!(someone.sign_in_count, 1);
        assert!(someone.active);
    }

    #[test]
    fn test_tuple_struct() {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
        assert_eq!(black.0 + black.1 + black.2, 0);
        assert_eq!(origin.0 + origin.1 + origin.2, 0);
    }

    #[test]
    fn test_rectangle() {
        let rectangle: Rectangle = Rectangle {
            width: 30,
            height: 50,
        };
        let big_rectangle = Rectangle {
            width: 40,
            height: 60,
        };
        assert_eq!(1500, area(&rectangle));
        // Rust has a feature called automatic referencing and dereferencing.
        // when you call a method with object.something(),
        // Rust automatically adds in &, &mut, or * so object matches the signature of the method.
        assert_eq!(1500, rectangle.area());
        assert!(big_rectangle.can_hold(&rectangle));
        let square = Rectangle::square(40);
        assert_eq!(square.area(), 1600);
    }
}
