fn main() {
    let mut my_list: Vec<i32> = vec![];

    for x in 0..=10 {
        my_list.push(x);
    }
    println!("Before: {:?}", my_list);
    let doubled: Vec<i32> = my_list.iter().map(|i| i * 2).collect();

    println!("After: {:?}", doubled);
}
