use clap::Parser;

#[derive(Parser)]
#[command(version = "1.0", about = "A word count program")]
struct Cli {
    #[arg(required = false)]
    string: Option<String>,

    #[arg(short='d', default_value_t=String::from(" "))]
    delimiter: String,
}

fn main() {
    let cli = Cli::parse();

    if let Some(string) = cli.string {
        let words: Vec<String> = string
            .split(&cli.delimiter)
            .map(|word| String::from(word))
            .collect();
        let count = words.len();
        println!("{count}");
    } else {
        eprintln!("No data");
        std::process::exit(1)
    }
}
