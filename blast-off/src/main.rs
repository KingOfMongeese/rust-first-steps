
fn blast_off_for() {
    let mut counts: Vec<i32>= vec![];
    for x in  1..=10{
        counts.push(x);
    }

    counts.iter().rev().for_each(|x| println!("{x}"));

    println!("For launching!");
}

fn blast_off_while() {
    let mut cnt = 10;
    while cnt > 0 {
        println!("{cnt}");
        cnt -= 1;
    }

    println!("While launching!")
}

fn main() {
    blast_off_for();
    blast_off_while();
}
