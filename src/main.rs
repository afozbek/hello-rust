fn main() {
    println!("Hello, world!");
    // print_numbers_to(5)
    let typed_numbers: [i32; 5] = [1, 2, 3, 4, 5];
    for i in 0..typed_numbers.len() {
        println!("{}", typed_numbers[i])
    }
}

fn print_numbers_to(number: u32) {
    for n in 1..number {
        println!("{}", is_even(n));
    }
}

fn is_even(number: u32) -> bool {
    return number % 2 == 0;
}
