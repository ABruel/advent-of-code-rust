param ([int] $day);
function New-Prompt ($day) {
  Start-Process "https://adventofcode.com/2021/day/$day"
  $filename = ".\src\prompts\{0:d2}.md" -f $day
  Write-Output "" > $filename
}
function New-Input ($day) {
  Start-Process "https://adventofcode.com/2021/day/$day/input"
  $filename = ".\src\inputs\{0:d2}.txt" -f $day
  Write-Output "" > $filename
}
function New-Solution ($day) {
  $filename = ".\src\solutions\day{0:d2}.rs" -f $day
  Write-Output "" > $filename
}

git checkout -b ("{0:d4}-{0:d2}" -f $day)
New-Solution $day;
New-Input $day;
New-Prompt $day;

