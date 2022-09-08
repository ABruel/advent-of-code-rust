use clap::Parser;

mod aoc;
mod solutions;

macro_rules! solve_part {
    ($x:expr, $i:expr) => {{
        use std::time::Instant;
        let now = Instant::now();
        let r = $x($i).unwrap();
        let elapsed = now.elapsed();
        println!("{} -- Elapsed: {:.2?}", r, elapsed);
    }};
}

macro_rules! solve {
    ($x:path, $d:expr) => {{
        use $x::*;
        let input = aoc::read($d);
        solve_part!(part_one, &input);
        solve_part!(part_two, &input);
    }};
}
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
        1 => solve!(solutions::day01, 1),
        2 => solve!(solutions::day02, 2),
        _ => unimplemented!("day {}", args.day),
    }
}
