#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

pub fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    pub fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Rectangle {
    fn summarize(&self) -> String {
        format!("Rect is size {} by {}", self.width, self.height)
    }
}

// if Generic can implement Summary
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn lifetimes() {

    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result: &str = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}

/// Compiler doesn't know if x or y have the same lifetime
/// Explicit lifetime notation for the var with shorter lifetime
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
