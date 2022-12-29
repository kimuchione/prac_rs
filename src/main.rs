fn take_value(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4]) //wrapped option<i32>
    }
}

fn main() {
    let first_vec = vec![1, 2, 3, 4, 5];
    let index = take_value(first_vec); //Option.unwrap() take out what is inside to unwrap function
    match index {
        Some(number) => println!("The number is {}", number),
        _ => println!("None"),
    };
}
