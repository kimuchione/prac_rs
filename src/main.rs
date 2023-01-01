use std::vec;

fn main() {
    let weather = vec![
        vec!["seoul", "sunny", "-4", "2"],
        vec!["busan", "sunny", "1", "10"],
    ];

    for mut i in weather {
        println!("{}", i[0]);
        while let Some(result) = i.pop() {
            //match의 모든 유효성 검사를 받을 필요가 없어짐
            if let Ok(number) = result.parse::<i32>() {
                println!("The number is {}", number);
            }
        }
    }
}
