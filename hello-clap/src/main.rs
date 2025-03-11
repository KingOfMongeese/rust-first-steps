
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version = "1.0", about = "A hello world cli")]
struct Cli {

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// echo a msg
    Echo {
        msg: String,

        #[clap(short = 'l', default_value_t = false, help = "include a new line or not")]
        extra_new_line: bool,
    },

    /// sum two numbers
    SumTwo {
        num1: i32,
        num2: i32,
    },
}

fn echo(s: String, extra_new_line: bool) {
    println!("{s}");
    if extra_new_line {
        println!();
    }
}

fn sum_two(x: i32, y: i32) {
    let sum = x + y;
    println!("{sum}");
}

fn main() {

    let cli = Cli::parse();

    match &cli.command {
        Commands::Echo { msg, extra_new_line } => echo(msg.to_string(), *extra_new_line),
        Commands::SumTwo { num1, num2 } => sum_two(*num1, *num2),
    }
}