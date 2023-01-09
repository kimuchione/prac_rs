fn main() {
    // 1 to 10 including 10
    let new_vec = (1..=10).collect::<Vec<_>>();

    let four = new_vec.clone().into_iter().skip(1).skip(2).take(2).next();

    if let Some(number) = four {
        println!("{number}");
    };

    let my_vec = new_vec
        .into_iter()
        .skip(1)
        .skip(2)
        .take(4)
        .collect::<Vec<i32>>();

    assert_eq!(my_vec, vec![4, 5, 6, 7]);
}
