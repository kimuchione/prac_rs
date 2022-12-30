fn is_positive_number(input: i32) -> Result<i32, String> {
    if input.is_positive() == true {
        Ok(1)
    } else {
        Err("No".to_string())
    }
}

fn main() {
    let mut result = Vec::new();

    for n in -5..5 {
        result.push(is_positive_number(n));
    }

    println!("{:?}", result);
}
