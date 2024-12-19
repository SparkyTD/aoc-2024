use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use crate::utils::solution::{solution, Solution};

fn fix_page_order(ordering_rules: &HashMap<u8, HashSet<u8>>, pages: &mut Vec<u8>) -> bool {
    let mut num_indices: HashMap<u8, u8> = HashMap::new();

    for i in 0..pages.len() {
        let page = pages[i];

        if let Some(after_list) = ordering_rules.get(&page) {
            for after in after_list {
                // `after` MUST come AFTER `page`
                // if `after` is already in `seen` then swap to fix
                if let Some(after_index) = num_indices.get(after) {
                    let after_index = *after_index as usize;

                    // swap current `page[i]` with previously seen `after[after_index]`
                    (pages[i], pages[after_index]) = (pages[after_index], pages[i]);
                    return fix_page_order(ordering_rules, pages) || true;
                }
            }
        }

        num_indices.insert(page, i as u8);
    }

    false
}

/// <b>Part 1:</b> Sum up the middle page numbers from every correctly-ordered update
/// <br/><br/><b>Part 2:</b> Fix incorrectly ordered updates, add up the middle page numbers
#[derive(Default)]
pub struct PrintQueue;

impl Solution for PrintQueue {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let mut ordering_rules: HashMap<u8, HashSet<u8>> = HashMap::new();
        let mut reading_page_list = false;

        let mut correct_middle_total: u32 = 0;
        let mut fixed_middle_total: u32 = 0;

        for line in input.lines() {
            if line.is_empty() {
                reading_page_list = true;
                continue;
            }

            if !reading_page_list {
                let numbers_str = line.split("|").collect::<Vec<&str>>();
                let x = numbers_str[0].parse::<u8>().unwrap();
                let y = numbers_str[1].parse::<u8>().unwrap();

                if !ordering_rules.contains_key(&x) {
                    ordering_rules.insert(x, HashSet::from([y]));
                } else {
                    ordering_rules.get_mut(&x).unwrap().insert(y);
                }
            } else {
                let mut numbers = line.split(",").collect::<Vec<&str>>()
                    .iter().map(|s| s.parse::<u8>().unwrap()).collect::<Vec<u8>>();

                let has_fixed = fix_page_order(&ordering_rules, &mut numbers);

                let middle = numbers[numbers.len() / 2] as u32;
                if has_fixed {
                    fixed_middle_total += middle;
                } else {
                    correct_middle_total += middle;
                }
            }
        }

        solution!(correct_middle_total, fixed_middle_total)
    }
}