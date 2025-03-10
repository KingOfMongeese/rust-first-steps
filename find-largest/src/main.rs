

fn find_largest<'a>(list: &'a Vec<i32>) -> &'a i32 {

    let mut max = &list[0];
    for num in list.iter().skip(1) {

        if *max < *num {
            max = num;
            println!("Largest is now: {}", max)
        }
    }

    max
}

fn main() {
    
    let my_list = vec![0, 1, 2, 3, 4, 5, 6, 20, 7, 8, 9, 10];
    let largest = find_largest(&my_list);
    let mut number = 40;

    println!("Largest Val: {}\nList: {:?}", largest, my_list);
    println!("Number before: {}", number);

    number += largest;
    println!("Number after: {}", number);
}
