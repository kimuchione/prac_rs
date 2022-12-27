fn main() {
    let mut num = 0;

    'first_iter: while num != 5 {
        //==tick make break point
        num += 1;
        println!("My number is {}", num);
        if num == 3 {
            'second_iter: loop {
                num += 5;
                break 'first_iter; //break first iteration
            }
        }
    }
}
