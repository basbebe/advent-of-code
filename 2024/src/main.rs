use clap::{Parser, Subcommand};

mod day01;
mod day02;
mod inputs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
    Day09,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    let solution = match &cli.command {
        Commands::Day01 => day01::run(inputs::DAY01),
        Commands::Day02 => todo!(),
        Commands::Day03 => todo!(),
        Commands::Day04 => todo!(),
        Commands::Day05 => todo!(),
        Commands::Day06 => todo!(),
        Commands::Day07 => todo!(),
        Commands::Day08 => todo!(),
        Commands::Day09 => todo!(),
        Commands::Day10 => todo!(),
        Commands::Day11 => todo!(),
        Commands::Day12 => todo!(),
        Commands::Day13 => todo!(),
        Commands::Day14 => todo!(),
        Commands::Day15 => todo!(),
        Commands::Day16 => todo!(),
        Commands::Day17 => todo!(),
        Commands::Day18 => todo!(),
        Commands::Day19 => todo!(),
        Commands::Day20 => todo!(),
        Commands::Day21 => todo!(),
        Commands::Day22 => todo!(),
        Commands::Day23 => todo!(),
        Commands::Day24 => todo!(),
    };

    match solution {
        Ok(()) => (),
        Err(e) => eprintln!("Error: {e}"),
    };
}
