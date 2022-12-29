use std::fmt::Display;

#[derive(Debug)]
struct Book;

fn return_func<T, U>(params: T, number1: U, number2: U)
where
    T: Display, // T is not keyword. If you want to write DisplayType? try it .
    U: Display + std::cmp::PartialOrd, // If you worried about typing over you put in generics on where
{
    let is_greater = if number1 >= number2 { true } else { false };
    println!(
        "Hey {}, Is {} greater than {} ? {} ",
        params, number1, number2, is_greater
    );
}

fn main() {
    return_func("jiwon", 10, 5);
}
