use clap::Parser;

#[derive(Parser)]
#[command(version = "1.0", about = "A word count program")]
struct Cli {
    #[arg(required = false)]
    string: Option<String>,

    #[arg(short='d', default_value_t=String::from(" "))]
    delimiter: String,
}

fn normalize_white_space(string: &mut String) {
    *string = string.trim().to_string();

    let mut prev = 'a';
    let mut temp_str = String::new();
    for c in string.chars() {
        if c == ' ' && prev == ' ' {
            prev = c;
            continue;
        }
        temp_str += &c.to_string();
    }

    *string = temp_str;
}

fn main() {
    let cli = Cli::parse();

    if let Some(mut string) = cli.string {
        normalize_white_space(&mut string);
        let words: Vec<String> = string
            .split(&cli.delimiter)
            .map(|word| String::from(word))
            .collect();
        let count = words.len();
        println!("{count}");
        println!("{:?}", words);
    } else {
        eprintln!("No data");
        std::process::exit(1)
    }
}
