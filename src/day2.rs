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
        let mut report = report.clone().to_vec();
        report.remove(i);
        if is_report_safe(&report) {
            return true;
        }
    }
    false
}

pub fn day_2() {
    let input = include_str!("../data/day2.txt");

    let mut safe_count = 0;
    let mut tolerance_safe_count = 0;
    for report_line in input.lines() {
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

    println!("Safe count: {}", safe_count);
    println!("Safe count (with tolerance): {}", safe_count + tolerance_safe_count);
}
