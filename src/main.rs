use clap::Parser;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Day to solve
    #[clap(short, long, value_parser)]
    day: u8,
}

fn main() {
    let args = Args::parse();
    match args.day {
        _ => unimplemented!("day {}", args.day),
    }
}
