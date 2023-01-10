fn main() {
    let number = 9;
    let anonymous_function = || println!("9");
    let closure = || println!("{number}");

    anonymous_function();
    closure();
}
