mod enumTest;
mod rect;
mod threading;

use std::{convert, io};

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut x = 5;
    x = 6;

    const MAX_POINTS: u32 = 100_000;

    // Shadowing, creates new variable
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    // Datatypes

    let guess: u32 = "42".parse().expect("Not a number!");
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let six_point_four = tup.1; // Index
    let (x, y, z) = tup;

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // a = [3, 3, 3, 3, 3];
    let second_elem = a[1];

    let spaces = "   ";
    let spaces = spaces.len();

    // std io

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: usize = input
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // Functions
    another_function(5);

    // Expressions
    let x = five();
    let y = {
        let x = 3;
        x + 1 // Note that there is no ;
    };

    // Control Flow

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4 or 3");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    // Loops return on break
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    // While and For loop
    let a = [10, 20, 30, 40, 50];

    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    // Ownership and Strings

    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    // s is not longer valid here as it is out of the scope

    let s1 = String::from("hello");
    // let s2 = s1;
    // Doing the above would copy the pointer, and cause issues with s1 dropping out of scope early
    // Therefore, you must make a clone (deep copy)
    let s2 = s1.clone();

    // But when it comes to the Stack, it copies automatically
    let x = 5;
    let y = x;

    // Ownership is taken in functions too, not just copied
    // * if is a pointer to a heap
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // Takes ownership, gives it back, moving s2 out

    // Reference and Borrowing

    let mut s1 = String::from("hello");
    let len = calculate_length(&s1); // Does not take ownership
    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1); // mutable reference

    // Slices also do not require ownership

    let s = String::from("hello world");

    let hello = &s[..5];
    let world = &s[6..11];
    let first_word_s = first_word(&s);

    // Structs

    // Instantiate
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // Struct

    let rect1 = rect::Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = rect::Rectangle {
        width: 20,
        height: 40,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect::area(&rect1)
    );

    println!("rect1 is {:#?}", rect1);
    println!("rect1 is {}", rect1.perimeter());
    println!("rect1 is {}", rect1.can_hold(&rect2));

    let square1 = rect::Rectangle::square(10);

    println!("rect1 is {:#?}", square1);

    enumTest::test();
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// You can't modify something you borrow
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5 // Missing semicolon acts as return
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    a_string // a_string is returned and moves out to the calling function
}
