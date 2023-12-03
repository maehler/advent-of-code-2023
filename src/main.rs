use clap::Parser;

mod day1;

#[derive(Parser, Debug)]
struct Args {
    day: u8,
    input: String,
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => day1::run(args.input),
        _ => println!("Day {} not implemented", args.day),
    }
}
