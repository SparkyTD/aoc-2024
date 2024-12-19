use std::fmt::Display;
use ahash::AHashMap;
use rayon::prelude::*;
use crate::utils::prefix_tree::PrefixTree;
use crate::utils::solution::{solution, Solution};

fn count_arrangements(string: &str, index: usize, prefix_tree: &PrefixTree, arrangements_memo: &mut AHashMap<usize, usize>) -> usize {
    if index >= string.len() {
        return 1;
    }

    let prefixes = prefix_tree
        .prefixes_of(&string[index..])
        .into_iter()
        .map(|p| p.len());

    let mut arrangement_count = 0;
    for pref_length in prefixes {
        let key = index + pref_length;
        if let Some(saved_arrangement_count) = arrangements_memo.get(&key) {
            if *saved_arrangement_count == 0 {
                continue;
            } else {
                arrangement_count += *saved_arrangement_count;
            }
        } else {
            let arrangements = count_arrangements(string, index + pref_length, &prefix_tree, arrangements_memo);
            arrangements_memo.insert(key, arrangements);
            arrangement_count += arrangements;
        }
    }

    arrangement_count
}

/// <b>Part 1:</b> Count the number of color sequences that can be arranged using the specified striped towels
/// <br/><br/><b>Part 2:</b> Sum up every possible towel arrangement for evey color sequence
#[derive(Default)]
pub struct LinenLayout;

impl Solution for LinenLayout {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let mut lines = input.lines();
        let prefix_tree = PrefixTree::from_vec(lines.next()
            .unwrap().split([',', ' '])
            .filter(|p| p.len() > 0)
            .collect::<Vec<&str>>());
        lines.next();
        let required_patterns = lines.collect::<Vec<_>>();

        let arrangement_counts = required_patterns
            .par_iter()
            .map(|pattern| {
                count_arrangements(pattern, 0, &prefix_tree, &mut AHashMap::default())
            })
            .collect::<Vec<usize>>();

        let possible_patterns = arrangement_counts.iter().filter(|count| **count > 0).count();
        let total_arrangements: usize = arrangement_counts.iter().sum();

        solution!(possible_patterns, total_arrangements)
    }
}