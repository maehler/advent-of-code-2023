use clap::Parser;

mod utils;
mod day1;
mod day2;
mod day3;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    part: u8,
    day: u8,
    input: String,
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => day1::run(args.part, args.input),
        2 => day2::run(args.part, args.input),
        3 => day3::run(args.part, args.input),
        _ => println!("Day {} not implemented", args.day),
    }
}
