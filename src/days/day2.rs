use std::fmt::Display;
use crate::utils::solution::{solution, Solution};

fn is_report_safe(report: &[u32]) -> bool {
    if report.len() == 0 {
        panic!("The report must contain at least one value");
    }

    let mut last_level = report[0];
    let increasing = {
        let mut inc_count = 0;
        let mut dec_count = 0;
        let mut last_level = report[0];
        for l in report[1..].iter() {
            if *l > last_level {
                inc_count += 1;
            } else if *l < last_level {
                dec_count += 1;
            }
            last_level = *l;
        }

        inc_count > dec_count
    };
    for l in report[1..].iter() {
        let diff = last_level.abs_diff(*l);
        if diff < 1 || diff > 3 || increasing && last_level > *l || !increasing && last_level < *l {
            return false;
        }

        last_level = *l;
    }

    true
}

fn is_report_safe_with_tolerance(report: &[u32]) -> bool {
    for i in 0..report.len() {
        let mut report = report.to_vec();
        report.remove(i);
        if is_report_safe(&report) {
            return true;
        }
    }
    false
}

/// <b>Part 1:</b> Count the number of safe reports that are in only increasing/decreasing order
/// <br/><br/><b>Part 2:</b> Count how many reports can be made safe by removing an element
#[derive(Default)]
pub struct RedNosedReports;

impl Solution for RedNosedReports {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let mut safe_count = 0;
        let mut tolerance_safe_count = 0;
        for report_line in input.lines() {
            if report_line.is_empty() {
                continue;
            }
            let levels: Vec<u32> = report_line
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            if is_report_safe(&levels) {
                safe_count += 1
            } else if is_report_safe_with_tolerance(&levels) {
                tolerance_safe_count += 1
            }
        }

        solution!(safe_count, safe_count + tolerance_safe_count)
    }
}