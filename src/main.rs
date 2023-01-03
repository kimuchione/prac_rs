use std::num::ParseIntError;

fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let parsed_number = input.parse::<i32>()?; //if parse failed then will be return ParseIntError
    Ok(parsed_number)
}

// It looks like optional chaining in javascript but js return only undefined. (too hard search debug selection)
// fn parse_str(inputL: &str) -> Result<(), ParseIntError> {
// let parsed_number = input.parse::<i32>()?;  ==> question mark operatror can use only return Result type
// println!("{}",parsed_number);
// Ok(())
// }

fn main() {
    for val in vec!["five", "8", "9.0", "four"] {
        println!("{:?}", parse_str(val));
    }
}
