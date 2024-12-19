# ❄️ Advent of Code 2024 ❄️
This is my first year participating in Advent of Code. I love a good challenge, and this event most certainly seems like one!
It is also part of my Rust learning journey, as I did not have a lot of experience with the language before starting this challenge.

## My solutions so far:
<!-- {RESULTS_START} -->
| Day | Status | Execution Time Comparison | Execution Time |
|-----|--------|---------------------------|----------------|
|[1](src/days/day1.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)| 1ms            |
|[2](src/days/day2.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)| 425µs          |
|[3](src/days/day3.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)| 115µs          |
|[4](src/days/day4.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/1/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)| 1ms            |
|[5](src/days/day5.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/11/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)| 17ms           |
|[6](src/days/day6.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/83/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)| 95ms           |
|[7](src/days/day7.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/51/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)| 76ms           |
|[8](src/days/day8.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)| 388µs          |
|[9](src/days/day9.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/41/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)| 61ms           |
|[10](src/days/day10.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)| 748µs          |
|[11](src/days/day11.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/7/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)| 10ms           |
|[12](src/days/day12.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/31/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)| 47ms           |
|[13](src/days/day13.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)| 80µs           |
|[14](src/days/day14.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/28/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)| 42ms           |
|[15](src/days/day15.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)| 839µs          |
|[16](src/days/day16.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/3/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)| 4ms            |
|[17](src/days/day17.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)| 54µs           |
|[18](src/days/day18.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/10/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)| 15ms           |
|[19](src/days/day19.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/1/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)| 2ms            |
<!-- {DAY 1=1238} -->
<!-- {DAY 2=425} -->
<!-- {DAY 3=115} -->
<!-- {DAY 4=1683} -->
<!-- {DAY 5=17587} -->
<!-- {DAY 6=124226} -->
<!-- {DAY 7=76287} -->
<!-- {DAY 8=388} -->
<!-- {DAY 9=61365} -->
<!-- {DAY 10=748} -->
<!-- {DAY 11=10554} -->
<!-- {DAY 12=47477} -->
<!-- {DAY 13=80} -->
<!-- {DAY 14=42459} -->
<!-- {DAY 15=839} -->
<!-- {DAY 16=4955} -->
<!-- {DAY 17=54} -->
<!-- {DAY 18=15248} -->
<!-- {DAY 19=2484} -->
<!-- {RESULTS_END} -->
*The table above is automatically generated with each execution of the test suite.*

## Test Inputs
To respect the rules of Advent of Code, as well as the event's creator, my personalized inputs are not included in this repository.
This project uses a custom test file format that allows multiple different test cases to be checked easily.
Each test is prefixed with `@test`, which is followed by the program input. To perform result checking,
the `@test` tag can be extended with `@part1 <PART1_RESULT>` and/or `@part2 <PART1_RESULT>`.

Here's an example test file (`./data/day17.test`)
```
@test
@part1 4,6,3,5,6,3,5,2,1,0
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0

@test
@part2 117440
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0

@test
@part1 [Redacted]
@part2 [Redacted]
Register A: [Redacted]
Register B: 0
Register C: 0

Program: [Redacted]
```