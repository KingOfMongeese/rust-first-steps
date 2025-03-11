use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        eprintln!("echo_clone: No msg");
        std::process::exit(1);
    }

    args.iter().skip(1).for_each(|s| print!("{s} "));
}
