use std::collections::{HashMap};
use std::fmt::Display;
use crate::utils::solution::{solution, Solution};

#[derive(Default, Debug)]
struct StoneConfiguration {
    zero_count: u64,
    even_digits: HashMap<u64, u64>,
    odd_digits: HashMap<u64, u64>,
}

impl StoneConfiguration {
    pub fn stone_count(&self) -> u64 {
        self.zero_count + self.even_digits.values().sum::<u64>() + self.odd_digits.values().sum::<u64>()
    }
}

fn get_digit_count(mut number: u64) -> u32 {
    let mut digits = 0;
    while number > 0 {
        number /= 10;
        digits += 1;
    }
    digits
}

fn increment_hashmap(map: &mut HashMap<u64, u64>, number: u64, add: u64) {
    if let Some(count) = map.get_mut(&number) {
        *count += add;
    } else {
        map.insert(number, add);
    }
}

fn split_number(number: u64) -> (u64, u64) {
    let digit_count = get_digit_count(number);
    assert_eq!(digit_count % 2, 0);

    let mut left = number;
    let mut right = 0;
    let mut multiplier = 1;

    for _ in 0..digit_count / 2 {
        right += (left % 10) * multiplier;
        multiplier *= 10;
        left /= 10;
    }

    (left, right)
}

fn insert_number(config: &mut StoneConfiguration, number: u64, count: u64) {
    if number == 0 {
        config.zero_count += count;
    } else if get_digit_count(number) % 2 == 0 {
        increment_hashmap(&mut config.even_digits, number, count);
    } else {
        increment_hashmap(&mut config.odd_digits, number, count);
    }
}

fn blink(old_config: StoneConfiguration) -> StoneConfiguration {
    let mut new_config = StoneConfiguration::default();

    // Convert 0 to 1
    increment_hashmap(&mut new_config.odd_digits, 1, old_config.zero_count);

    // Process even digit numbers
    for (number, count) in &old_config.even_digits {
        let (num1, num2) = split_number(*number);
        insert_number(&mut new_config, num1, *count);
        insert_number(&mut new_config, num2, *count);
    }

    // Process other numbers
    for (number, count) in &old_config.odd_digits {
        let new_number = number * 2024;
        insert_number(&mut new_config, new_number, *count);
    }

    new_config
}

/// <b>Part 1:</b> Simulate numbered stone rearrangements 50 times
/// <br/><br/><b>Part 2:</b> Simulate the same rearrangements 75 times
#[derive(Default)]
pub struct PlutonianPebbles;

impl Solution for PlutonianPebbles {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let mut config = StoneConfiguration::default();

        for number in input.split_whitespace().map(|s| s.parse::<u64>()) {
            if let Ok(number) = number {
                insert_number(&mut config, number, 1);
            }
        }

        for _ in 0..25 {
            config = blink(config);
        }
        let part1 = config.stone_count();

        for _ in 0..50 {
            config = blink(config);
        }
        let part2 = config.stone_count();

        solution!(part1, part2)
    }
}