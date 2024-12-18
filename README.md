# ❄️ Advent of Code 2024 ❄️
This is my first year participating in Advent of Code. I love a good challenge, and this event most certainly seems like one!
It is also part of my Rust learning journey, as I did not have a lot of experience with the language before starting this challenge.

## My solutions so far:
<!-- {RESULTS_START} -->
| Day | Status | Execution Time Comparison | Execution Time |
|-----|--------|---------------------------|----------------|
|[Day 1](src/days/day1.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|644µs|
|[Day 2](src/days/day2.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|210µs|
|[Day 3](src/days/day3.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|88µs|
|[Day 4](src/days/day4.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|2ms|
|[Day 5](src/days/day5.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/2/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|17ms|
|[Day 6](src/days/day6.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/83/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|543ms|
|[Day 7](src/days/day7.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/10/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|71ms|
|[Day 8](src/days/day8.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|215µs|
|[Day 9](src/days/day9.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/9/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|59ms|
|[Day 10](src/days/day10.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|730µs|
|[Day 11](src/days/day11.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/1/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|10ms|
|[Day 12](src/days/day12.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/7/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|49ms|
|[Day 13](src/days/day13.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|146µs|
|[Day 14](src/days/day14.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/6/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|44ms|
|[Day 15](src/days/day15.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|964µs|
|[Day 16](src/days/day16.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|5ms|
|[Day 17](src/days/day17.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|55µs|
|[Day 18](src/days/day18.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/2/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|15ms|
<!-- {DAY 1=644} -->
<!-- {DAY 2=210} -->
<!-- {DAY 3=88} -->
<!-- {DAY 4=2132} -->
<!-- {DAY 5=17343} -->
<!-- {DAY 6=543903} -->
<!-- {DAY 7=71558} -->
<!-- {DAY 8=215} -->
<!-- {DAY 9=59179} -->
<!-- {DAY 10=730} -->
<!-- {DAY 11=10528} -->
<!-- {DAY 12=49160} -->
<!-- {DAY 13=146} -->
<!-- {DAY 14=44305} -->
<!-- {DAY 15=964} -->
<!-- {DAY 16=5194} -->
<!-- {DAY 17=55} -->
<!-- {DAY 18=15772} -->
<!-- {RESULTS_END} -->
*The table above is automatically generated with each execution of the test suite.*

## Test Inputs
To respect the rules of Advent of Code, as well as the event's creator, my personalized inputs are not included in this repository.
This project uses a custom test file format that allows multiple different test cases to be checked easily.
Each test is prefixed with `@test`, which is followed by the program input. To perform result checking,
the `@test` tag can be extended with `@part1 <PART1_RESULT>` and/or `@part2 <PART1_RESULT>`.

Here's an example test file (`day26.test`)
```
@test
@part1 1234
some-multiline
program-input

@test
@part2 esrever
rev(reverse)

@test
@part1 12
@part3 34
split 1234
```