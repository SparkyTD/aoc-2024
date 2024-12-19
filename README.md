# ❄️ Advent of Code 2024 ❄️
This is my first year participating in Advent of Code. I love a good challenge, and this event most certainly seems like one!
It is also part of my Rust learning journey, as I did not have a lot of experience with the language before starting this challenge.

## My solutions so far:
<!-- {RESULTS_START} -->
| Day | Status | Execution Time Comparison | Execution Time |
|-----|--------|---------------------------|----------------|
|[1](src/days/day1.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|673µs|
|[2](src/days/day2.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|216µs|
|[3](src/days/day3.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|87µs|
|[4](src/days/day4.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/1/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|1ms|
|[5](src/days/day5.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/14/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|17ms|
|[6](src/days/day6.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/83/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|105ms|
|[7](src/days/day7.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/57/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|73ms|
|[8](src/days/day8.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|215µs|
|[9](src/days/day9.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/51/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|64ms|
|[10](src/days/day10.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|843µs|
|[11](src/days/day11.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/9/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|11ms|
|[12](src/days/day12.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/46/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|58ms|
|[13](src/days/day13.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|96µs|
|[14](src/days/day14.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/39/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|50ms|
|[15](src/days/day15.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/1/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|1ms|
|[16](src/days/day16.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/7/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|9ms|
|[17](src/days/day17.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|154µs|
|[18](src/days/day18.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/13/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|16ms|
|[19](src/days/day19.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/2/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|3ms|
<!-- {DAY 1=673} -->
<!-- {DAY 2=216} -->
<!-- {DAY 3=87} -->
<!-- {DAY 4=1477} -->
<!-- {DAY 5=17690} -->
<!-- {DAY 6=105285} -->
<!-- {DAY 7=73041} -->
<!-- {DAY 8=215} -->
<!-- {DAY 9=64683} -->
<!-- {DAY 10=843} -->
<!-- {DAY 11=11402} -->
<!-- {DAY 12=58320} -->
<!-- {DAY 13=96} -->
<!-- {DAY 14=50281} -->
<!-- {DAY 15=1509} -->
<!-- {DAY 16=9846} -->
<!-- {DAY 17=154} -->
<!-- {DAY 18=16637} -->
<!-- {DAY 19=3031} -->
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