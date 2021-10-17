use std::io;

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
    let s2 = String::from("hello");     // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // Takes ownership, gives it back, moving s2 out

}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5 // Missing semicolon acts as return
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    a_string  // a_string is returned and moves out to the calling function
}
