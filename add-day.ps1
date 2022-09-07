param ([int] $day, [int] $year);
function New-Prompt ($year, $day) {
  $filename = ".\src\prompts\{0:d4}\{1:d2}.md" -f $year, $day
  Write-Output $filename
}
function New-Input ($year, $day) {
  $filename = ".\src\inputs\{0:d4}\{1:d2}.txt" -f $year, $day
  Write-Output "" > $filename
}
function New-Solution ($year, $day) {
  $filename = ".\src\solutions\{0:d4}\day{1:d2}.rs" -f $year, $day
  Write-Output $filename
}

git checkout -b "advent-of-code $year day $day" -f $day
New-Solution $year $day
New-Input $year $day;
New-Prompt $year $day;
