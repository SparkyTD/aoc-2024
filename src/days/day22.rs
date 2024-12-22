use std::fmt::Display;
use rayon::prelude::*;
use ahash::{AHashMap};
use crate::utils::solution::{solution, Solution};

#[derive(Default)]
pub struct MonkeyMarket;

fn next_secret_number(mut secret: usize) -> usize {
    secret ^= secret << 6;
    secret %= 16777216; // 2^24
    secret ^= secret >> 5;
    secret %= 16777216; // 2^24
    secret ^= secret << 11;
    secret %= 16777216; // 2^24
    secret
}

fn encode_sequence(d1: i8, d2: i8, d3: i8, d4: i8) -> u32 {
    let result: u32 = (((d1.abs() & 0xf) as u32) << 16)
        | (((d2.abs() & 0xf) as u32) << 12)
        | (((d3.abs() & 0xf) as u32) << 8)
        | (((d4.abs() & 0xf) as u32) << 4);

    let signs = ((if d1 < 0 { 1 } else { 0 }) << 3)
        | ((if d2 < 0 { 1 } else { 0 }) << 2)
        | ((if d3 < 0 { 1 } else { 0 }) << 1)
        | (if d4 < 0 { 1 } else { 0 });

    result | signs
}

impl Solution for MonkeyMarket {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let seeds = input.lines().map(|l| l.parse::<usize>().unwrap()).collect::<Vec<usize>>();

        let best_price_sequences = seeds.par_iter()
            .map(|seed| {
                let mut seed = *seed;
                let mut last_digit: i8 = (seed % 10) as i8;
                let mut last_sequence = (0i8, 0i8, 0i8, 0i8);
                let mut sequence_best_prices = AHashMap::<_, u8>::new();
                for i in 0..2000 {
                    seed = next_secret_number(seed);
                    let diff = (seed % 10) as i8 - last_digit;
                    last_digit = (seed % 10) as i8;
                    last_sequence = (last_sequence.1, last_sequence.2, last_sequence.3, diff);
                    let sequence_code = encode_sequence(last_sequence.0, last_sequence.1, last_sequence.2, last_sequence.3);

                    if i >= 3 {
                        sequence_best_prices.entry(sequence_code).or_insert(last_digit as u8);
                    }
                }

                (sequence_best_prices, seed)
            }).collect::<Vec<_>>();

        let mut p1_sum = 0;
        let mut sum_map = AHashMap::<_, u64>::new();
        let mut best_total = 0;
        for map in best_price_sequences {
            p1_sum += map.1;
            for (key, &value) in &map.0 {
                let old_value = sum_map.entry(key.clone()).or_insert(0);
                *old_value += value as u64;
                if *old_value > best_total {
                    best_total = *old_value;
                }
            }
        }

        solution!(p1_sum, best_total)
    }
}