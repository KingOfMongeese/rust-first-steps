
fn swap<'x>(mut a: &'x mut i32, mut b: &'x mut i32) -> (&'x i32, &'x i32) {

    let temp = a;
    a = b;
    b = temp;

    (a, b)

}

fn main() {
    
    let mut a = 0;
    let mut b = 1;

    println!("Before:\nA: {}\nB: {}", a, b);

    let swapped = swap(&mut a, &mut b);

    println!("After:\nA: {}\nB: {}", swapped.0, swapped.1);
}

// correct rust way would be to use std::mem::swap
