use std::fmt::Display;
use rayon::prelude::*;
use crate::utils::solution::{solution, Solution};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operator {
    Addition,
    Multiplication,
    Concatenation,
}

fn apply_operator(a: u64, b: u64, operator: Operator) -> u64 {
    match operator {
        Operator::Addition => a + b,
        Operator::Multiplication => a * b,
        Operator::Concatenation => {
            let mut b2 = b;
            let mut multiplier = 1;
            while b2 > 0 {
                multiplier *= 10;
                b2 /= 10;
            }

            a * multiplier + b
        }
    }
}

fn next_operator(operator: Operator, enable_concat: bool) -> Operator {
    match operator {
        Operator::Addition => Operator::Multiplication,
        Operator::Multiplication => if enable_concat { Operator::Concatenation } else { Operator::Addition },
        Operator::Concatenation => Operator::Addition,
    }
}

fn increment_operator_array(operators: &mut [Operator], enable_concat: bool) -> bool {
    if operators.len() == 0 {
        return false;
    }

    operators[0] = next_operator(operators[0], enable_concat);

    if operators[0] == Operator::Addition {
        return increment_operator_array(&mut operators[1..], enable_concat);
    }

    true
}

fn evaluate_expression(numbers: &[u64], operators: &[Operator]) -> u64 {
    assert_eq!(numbers.len(), operators.len() + 1);

    let mut result: u64 = numbers[0];

    let mut i = 1;
    for op in operators {
        result = apply_operator(result, numbers[i], *op);
        i += 1;
    }

    result
}

fn test_input(result: u64, components: &[u64], enable_concat: bool) -> bool {
    let mut operator_array: Vec<Operator> = Vec::new();
    for _ in 1..components.len() {
        operator_array.push(Operator::Addition);
    }

    loop {
        let test_result = evaluate_expression(components, &operator_array);
        if test_result == result {
            return true;
        }

        if !increment_operator_array(&mut operator_array, enable_concat) {
            break;
        }
    }

    false
}

/// <b>Part 1:</b> Find the equations that can be solved with a sequence of (+) and (-) operations, sum them up
/// <br/><br/><b>Part 2:</b> Syn up every solvable equations with the additional concatenation operation
#[derive(Default)]
pub struct BridgeRepair;

impl Solution for BridgeRepair {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let results = input.lines()
            .collect::<Vec<&str>>()
            .par_iter()
            .map(|line| {
                let mut split = line.split(": ");
                let result = split.next().and_then(|s| s.parse::<u64>().ok()).unwrap();
                let components = split.next().unwrap().split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();

                let mut sum_part1 = 0;
                let mut sum_part2 = 0;

                if test_input(result, &components, false) {
                    sum_part1 += result;
                }

                if test_input(result, &components, true) {
                    sum_part2 += result;
                }

                (sum_part1, sum_part2)
            })
            .collect::<Vec<_>>();

        let sum_part1 = results.iter().map(|p| p.0).sum::<u64>();
        let sum_part2 = results.iter().map(|p| p.1).sum::<u64>();

        solution!(sum_part1, sum_part2)
    }
}