# ❄️ Advent of Code 2024 ❄️
This is my first year participating in Advent of Code. I love a good challenge, and this event most certainly seems like one!
It is also part of my Rust learning journey, as I did not have a lot of experience with the language before starting this challenge.

## My solutions so far:
<!-- {RESULTS_START} -->
| Day | Status | Execution Time Comparison | Execution Time |
|-----|--------|---------------------------|----------------|
|[1](src/days/day1.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|1ms|
|[2](src/days/day2.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|501µs|
|[3](src/days/day3.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|230µs|
|[4](src/days/day4.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|2ms|
|[5](src/days/day5.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/2/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|19ms|
|[6](src/days/day6.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/83/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|758ms|
|[7](src/days/day7.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/8/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|80ms|
|[8](src/days/day8.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|232µs|
|[9](src/days/day9.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/6/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|59ms|
|[10](src/days/day10.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|1ms|
|[11](src/days/day11.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/1/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|10ms|
|[12](src/days/day12.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/5/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|50ms|
|[13](src/days/day13.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|89µs|
|[14](src/days/day14.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/6/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|56ms|
|[15](src/days/day15.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|1ms|
|[16](src/days/day16.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|7ms|
|[17](src/days/day17.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/0/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|91µs|
|[18](src/days/day18.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/1/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|16ms|
<!-- {DAY 1=1611} -->
<!-- {DAY 2=501} -->
<!-- {DAY 3=230} -->
<!-- {DAY 4=2731} -->
<!-- {DAY 5=19985} -->
<!-- {DAY 6=758547} -->
<!-- {DAY 7=80883} -->
<!-- {DAY 8=232} -->
<!-- {DAY 9=59227} -->
<!-- {DAY 10=1092} -->
<!-- {DAY 11=10683} -->
<!-- {DAY 12=50847} -->
<!-- {DAY 13=89} -->
<!-- {DAY 14=56706} -->
<!-- {DAY 15=1302} -->
<!-- {DAY 16=7903} -->
<!-- {DAY 17=91} -->
<!-- {DAY 18=16378} -->
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