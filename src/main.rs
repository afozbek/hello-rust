use std::{fs::File, io::ErrorKind, panic::catch_unwind};

fn main() {
    hello_world();
}

fn hello_world() {
    println!("Hello World!");
}
fn print_numbers_to(number: u32) {
    for n in 1..number {
        println!("{}", is_even(n));
    }
}
fn is_even(number: u32) -> bool {
    return number % 2 == 0;
}
fn listNumbers() {
    let typed_numbers: [i32; 5] = [1, 2, 3, 4, 5];
    for i in 0..typed_numbers.len() {
        println!("{}", typed_numbers[i])
    }
}
fn test_vectors() {
    let mut v = vec![1, 2, 3, 4, 5];

    v.push(1);
    v.push(2);
    v.push(3);

    let first = &v[0];

    println!("The first element is: {}", first);
}
fn test_match_operator(number: i32) {
    match number {
        1 => println!("Number is one"),
        2 | 3 => println!("Number is two or three"),
        4..=10 => println!("It is between 4 to 10"),
        _ => println!("I dont know!!"),
    }
}
fn panic_test() {
    let file = File::open("index.txt");

    let f = match file {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("index.txt") {
                Ok(file) => file,
                Err(err) => panic!("Problem while creating a file {}", err),
            },
            other_error => {
                panic!("Problem opening the file {}", other_error)
            }
        },
    };
}
