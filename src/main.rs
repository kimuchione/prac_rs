#[derive(Debug)]
struct Book;

fn return_func<T: std::fmt::Display>(params: T) -> T {
    //declare generic
    println!("The thing is {}", params);
    params
}

fn main() {
    let x = return_func(5);
    let y = return_func(String::from("Hello"));
    let z = return_func(Book); // may not compiled
}
