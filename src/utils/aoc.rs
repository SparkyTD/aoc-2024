use std::collections::HashMap;
use std::time::Duration;
use chrono::Datelike;
use colored::Colorize;
use terminal_size::Width;
use crate::utils::solution::{Solution, SolveTest};
use crate::utils::test_set::{TestRunResult, PRINT_RESULTS};

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
        let prev_print_results = PRINT_RESULTS.get();
        PRINT_RESULTS.replace(false);
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

        let mut longest_duration = results.values()
            .filter_map(|value| value.as_ref())
            .map(|result| result.elapsed)
            .max().unwrap_or(Duration::from_secs(0));

        if longest_duration.as_secs() < 5 {
            longest_duration *= 2;
        }

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
            let progress_label = match result {
                None => "",
                Some(result) => &format_progress_bar(&result.elapsed, &longest_duration),
            };
            println!("   Day {: >2}: {} {}  {}", format!("{}", day).purple().bold(), status_label, progress_label, duration_label);
        }
        PRINT_RESULTS.replace(prev_print_results);

        self.check_date_and_print_link();
    }

    fn check_date_and_print_link(&self) {
        let local_date = chrono::Utc::now() - chrono::Duration::hours(7);
        if local_date.month() != 12 {
            return;
        }

        let mut keys = self.solutions.keys().collect::<Vec<&u8>>();
        keys.sort();

        let last_day_with_solution = **keys.last().unwrap() as u32;

        if local_date.day() > last_day_with_solution && last_day_with_solution < 25 {
            let next_day = last_day_with_solution + 1;
            let problem_url = format!("https://adventofcode.com/{}/day/{}", local_date.year(), next_day);
            println!();
            println!("{}: {}", "Your next AoC problem is ready! Grab it here".purple().bold().italic(), problem_url);
        }
    }
}

fn format_progress_bar(current_duration: &Duration, max_duration: &Duration) -> String {
    let terminal_width = terminal_size::terminal_size()
        .and_then(|(Width(w), terminal_size::Height(_))| Some(w as usize))
        .unwrap_or(120);

    let bar_width = terminal_width / 4;
    let progress = current_duration.as_secs_f64() / max_duration.as_secs_f64();
    let filled_width = (progress * bar_width as f64).round() as usize;

    let filled_bar = "=".repeat(filled_width);
    let empty_bar = " ".repeat(bar_width.saturating_sub(filled_width));

    format!("[{}{}{}{}]", "=".purple(), filled_bar.purple(), ">".bright_purple(), empty_bar)
}

pub fn format_elapsed(duration: Duration) -> String {
    let millis = duration.as_millis();
    let micros = duration.as_micros();
    let seconds = duration.as_secs();
    if micros < 1000 {
        format!("{}µs", micros).bright_cyan().to_string()
    } else if millis < 1000 {
        format!("{}ms", millis).bright_green().to_string()
    } else if seconds <= 5 {
        format!("{}s {}ms", duration.as_secs(), millis % 1000).yellow().bold().to_string()
    } else {
        format!("{}s {}ms", duration.as_secs(), millis % 1000).red().bold().to_string()
    }
}