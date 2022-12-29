use std::fmt::Display;

#[derive(Debug)]
struct Book;

fn return_func<T: Display, U: Display + std::cmp::PartialOrd>(params: T, number1: U, number2: U) {
    let is_greater = if number1 >= number2 { true } else { false };
    println!(
        "Hey {}, Is {} greater than {} ? {} ",
        params, number1, number2, is_greater
    );
}

fn main() {
    return_func("jiwon", 10, 5);
}
