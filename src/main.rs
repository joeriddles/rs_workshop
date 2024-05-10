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
    Add2(Arguments),
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
        Command::Add2(input) => {
            args = input;
            func = add2;
        }
        Command::Sub(input) => {
            args = input;
            func = |x, y| x - y; // inline closure
        }
    };

    let answer = func(args.lhs, args.rhs);
    println!("{answer}");
}

// one way of using generics
fn add<T: std::ops::Add<Output = T>>(lhs: T, rhs: T) -> T {
    lhs + rhs
}

// another way of using generics
fn add2<T>(lhs: T, rhs: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    lhs + rhs
}
