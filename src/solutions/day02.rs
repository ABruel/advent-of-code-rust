use itertools::Itertools;

use crate::aoc::parse_int;

pub fn part_one(input: &str) -> Result<i32, ()> {
    let (depth, pos) = parse_input(&input)
        .iter()
        .fold((0, 0), |(depth, pos), cur| match cur {
            Movement::Up(x) => (depth - x, pos),
            Movement::Down(x) => (depth + x, pos),
            Movement::Forward(x) => (depth, pos + x),
        });
    Ok(depth * pos)
}

pub fn part_two(input: &str) -> Result<i32, ()> {
    let (depth, pos, _) = parse_input(&input)
        .iter()
        .fold((0, 0, 0), |(depth, pos, aim), cur| match cur {
            Movement::Up(x) => (depth, pos, aim - x),
            Movement::Down(x) => (depth, pos, aim + x),
            Movement::Forward(x) => (depth + (aim * x), pos + x, aim),
        });
    Ok(depth * pos)
}

fn parse_input(input: &str) -> Vec<Movement> {
    input.lines().map(parse_movement).collect_vec()
}

fn parse_movement(line: &str) -> Movement {
    let mut spl = line.split(' ');
    let m = spl.next().unwrap();
    let i = parse_int(spl.next().unwrap());
    match m {
        "up" => Movement::Up(i),
        "down" => Movement::Down(i),
        "forward" => Movement::Forward(i),
        _ => panic!("invalid movement {}", m),
    }
}

enum Movement {
    Up(i32),
    Down(i32),
    Forward(i32),
}

#[cfg(test)]
mod test_day {
    use super::*;
    use crate::aoc::read_example;

    #[test]
    fn test_part_one() {
        let input = read_example(2);
        assert_eq!(part_one(&input).unwrap(), 150);
    }
    #[test]
    fn test_part_two() {
        let input = read_example(2);
        assert_eq!(part_two(&input).unwrap(), 900);
    }
}
