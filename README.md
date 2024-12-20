# ❄️ Advent of Code 2024 ❄️
This is my first year participating in Advent of Code. I love a good challenge, and this event most certainly seems like one!
It is also part of my Rust learning journey, as I did not have a lot of experience with the language before starting this challenge.

## My solutions so far:
<!-- {RESULTS_START} -->
| Day | Status | Execution Time Comparison | Execution Time |
|-----|--------|---------------------------|----------------|
|[1](src/days/day1.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|771µs|
|[2](src/days/day2.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|232µs|
|[3](src/days/day3.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|165µs|
|[4](src/days/day4.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/1/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|1ms|
|[5](src/days/day5.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/12/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|18ms|
|[6](src/days/day6.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/83/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|122ms|
|[7](src/days/day7.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/61/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|90ms|
|[8](src/days/day8.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|303µs|
|[9](src/days/day9.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/48/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|71ms|
|[10](src/days/day10.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|752µs|
|[11](src/days/day11.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/7/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|10ms|
|[12](src/days/day12.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/33/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|48ms|
|[13](src/days/day13.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|82µs|
|[14](src/days/day14.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/28/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|41ms|
|[15](src/days/day15.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|853µs|
|[16](src/days/day16.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/8/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|12ms|
|[17](src/days/day17.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|92µs|
|[18](src/days/day18.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/12/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|18ms|
|[19](src/days/day19.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/2/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|4ms|
<!-- {DAY 1=771} -->
<!-- {DAY 2=232} -->
<!-- {DAY 3=165} -->
<!-- {DAY 4=1505} -->
<!-- {DAY 5=18917} -->
<!-- {DAY 6=122532} -->
<!-- {DAY 7=90239} -->
<!-- {DAY 8=303} -->
<!-- {DAY 9=71247} -->
<!-- {DAY 10=752} -->
<!-- {DAY 11=10732} -->
<!-- {DAY 12=48525} -->
<!-- {DAY 13=82} -->
<!-- {DAY 14=41817} -->
<!-- {DAY 15=853} -->
<!-- {DAY 16=12682} -->
<!-- {DAY 17=92} -->
<!-- {DAY 18=18900} -->
<!-- {DAY 19=4004} -->
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