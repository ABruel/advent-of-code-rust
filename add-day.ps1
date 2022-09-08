param ([int] $day);
function New-Prompt ($day) {
  Start-Process "https://adventofcode.com/2021/day/$day"
  $filename = ".\src\prompts\{0:d2}.md" -f $day
  Write-Output "" > $filename
}
function New-Input ($day) {
  $filename = ".\src\inputs\{0:d2}.txt" -f $day
  Write-Output "" > $filename
}

function New-Example ($day) {
  Start-Process "https://adventofcode.com/2021/day/$day/input"
  $filename = ".\src\examples\{0:d2}.txt" -f $day
  Write-Output "" > $filename
}
function New-Solution ($day) {
  $filename = ".\src\solutions\day{0:d2}.rs" -f $day
  $template = @"
use itertools::Itertools;

pub fn part_one(input: &str) -> Result<i32, ()> {
    Err(())
}

pub fn part_two(input: &str) -> Result<i32, ()> {
    Err(())
}

#[cfg(test)]
mod test_day {
    use super::*;
    use crate::aoc::read_example;

    #[test]
    fn test_part_one() {
        let input = read_example(1);
        assert_eq!(part_one(&input).unwrap(), 0);
    }
    #[test]
    fn test_part_two() {
        let input = read_example(1);
        assert_eq!(part_two(&input).unwrap(), 0);
    }
}
"@;
  Write-Output $template > $filename
}

#git checkout -b ("Day {0:d2}" -f $day)
New-Solution $day;
New-Input $day;
New-Prompt $day;
New-Example $day;
