use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Add(Arguments),
    Sub(Arguments),
    Mul(Arguments),
    Div(Arguments),
}

#[derive(Debug, Args)]
struct Arguments {
    lhs: f32,
    rhs: f32,
}

fn main() {
    let cli = Cli::parse();
    let answer = match cli.command {
        Command::Add(args) => args.lhs + args.rhs,
        Command::Sub(args) => args.lhs - args.rhs,
        Command::Mul(args) => args.lhs * args.rhs,
        Command::Div(args) => args.lhs / args.rhs,
    };
    println!("{answer}");
}
