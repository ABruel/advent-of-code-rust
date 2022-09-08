use itertools::Itertools;

use crate::aoc::parse_int;

pub fn part_one(input: &str) -> Result<i32, ()> {
    Ok(sum_value_increases(&parse_input(input)))
}

pub fn part_two(input: &str) -> Result<i32, ()> {
    Ok(sum_value_increases(
        &parse_input(input)
            .windows(3)
            .map(calculate_window_value)
            .collect(),
    ))
}

fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(parse_int).collect_vec()
}
fn sum_value_increases(vec: &Vec<i32>) -> i32 {
    let (_, count) = vec.iter().fold((i32::MAX, 0), |(prev, count), cur| {
        (*cur, if *cur > prev { count + 1 } else { count })
    });
    count
}

fn calculate_window_value(window: &[i32]) -> i32 {
    window.iter().sum()
}

#[cfg(test)]
mod test_day1 {
    use super::*;
    use crate::aoc::read_example;

    #[test]
    fn test_part_one() {
        let input = read_example(1);
        assert_eq!(part_one(&input).unwrap(), 7);
    }
    #[test]
    fn test_part_two() {
        let input = read_example(1);
        assert_eq!(part_two(&input).unwrap(), 5);
    }
}
