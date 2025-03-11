
fn rev_string(str: &mut String) {
    let reversed: String = str.chars().rev().collect();
    *str = reversed;
}

fn every_other_char(str: &mut String) {

    let mut every_other = String::new();

    for (cnt, chr) in str.chars().enumerate() {
        if cnt % 2 == 0 {
            every_other.push(chr);
        }
    }

    *str = every_other;
}

fn main() {
    
    let mut str = String::from("Hello World");

    rev_string(&mut str);
    println!("{}", str);
    rev_string(&mut str);

    println!("{str}");
    every_other_char(&mut str);
    println!("{str}");

}
