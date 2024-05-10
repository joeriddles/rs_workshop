use clap::{Args, Parser, Subcommand};

type Num = f32;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Add(Arguments),
    Sub(Arguments),
    // Mul(Arguments),
    // Div(Arguments),
}

#[derive(Debug, Args)]
struct Arguments {
    lhs: Num,
    rhs: Num,
}

fn main() {
    let cli = Cli::parse();
    let args;
    let func: fn(Num, Num) -> Num;

    match cli.command {
        Command::Add(input) => {
            args = input;
            func = add;
        }
        Command::Sub(input) => {
            args = input;
            func = |x, y| x - y; // inline closure
        }
    };

    let answer = func(args.lhs, args.rhs);
    println!("{answer}");
}

fn add(lhs: Num, rhs: Num) -> Num {
    lhs + rhs
}
