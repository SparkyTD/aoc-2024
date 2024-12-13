use std::collections::HashMap;
use std::time::Duration;
use colored::Colorize;
use crate::utils::solution::{Solution, SolveTest};
use crate::utils::test_set::TestRunResult;

#[derive(Default)]
pub struct AdventOfCode {
    solutions: HashMap<u8, Box<dyn Solution>>,
}

impl AdventOfCode {
    pub fn add_solution(&mut self, day: u8, solution: Box<dyn Solution>) {
        if let Some(_) = self.solutions.insert(day, solution) {
            panic!("A solution has already been added for day {day}!")
        }
    }

    #[allow(dead_code)]
    pub fn solve_day(&self, day: u8, solve_test: SolveTest) {
        if let Some(solution) = self.solutions.get(&day) {
            solution.solve_test(day, solve_test);
        } else {
            panic!("No solution exists for day {day}!");
        }
    }

    #[allow(dead_code)]
    pub fn solve_all(&self) {
        let mut keys = self.solutions.keys().collect::<Vec<&u8>>();
        keys.sort();

        let mut results: HashMap<u8, Option<TestRunResult>> = HashMap::new();

        for day in &keys {
            let solution = self.solutions.get(day).unwrap();
            println!("Running solution for day {day}...");
            let result = solution.solve_test(**day, SolveTest::Last);
            results.insert(**day, result);
        }

        println!();

        println!("All solutions have been executed, here are the results:");
        for day in &keys {
            let result = results.get(day).unwrap();
            let status_label = match result {
                None => "[Inconclusive]".white(),
                Some(result) => match (result.part1_success, result.part2_success) {
                    (Some(true), Some(true)) | (Some(true), None) | (None, Some(true)) => "[Success]".bright_green().bold(),
                    (Some(false), _) | (_, Some(false)) => "[Fail]".red().bold(),
                    (None, None) => "[Inconclusive]".white(),
                }
            };
            let duration_label = match result {
                None => "",
                Some(result) => &format_elapsed(result.elapsed),
            };
            println!("   Day {: >2}: {} ({})", format!("{}", day).purple().bold(), status_label, duration_label);
        }
    }
}

pub fn format_elapsed(duration: Duration) -> String {
    let millis = duration.as_millis();
    let micros = duration.as_micros();
    if micros < 1000 {
        format!("{}µs", micros)
    } else if millis < 1000 {
        format!("{}ms", millis)
    } else {
        format!("{}s {}ms", duration.as_secs(), millis % 1000)
    }
}