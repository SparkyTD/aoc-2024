use crate::test_set::TestSet;

fn solve_linear_eq(a1: i64, b1: i64, c1: i64, a2: i64, b2: i64, c2: i64) -> (i64, i64) {
    let d = a1 * b2 - a2 * b1;
    let dx = c1 * b2 - c2 * b1;
    let dy = a1 * c2 - a2 * c1;

    let x = dx / d;
    let y = dy / d;

    (x, y)
}

pub fn day_13() {
    let test_set = TestSet::from(include_str!("../../data/day13.test"));
    test_set.test_all(|input| {
        let mut button_a_x: i64 = 0;
        let mut button_a_y: i64 = 0;
        let mut button_b_x: i64 = 0;
        let mut button_b_y: i64 = 0;
        let mut total_tokens_p1: i64 = 0;
        let mut total_tokens_p2: i64 = 0;
        for line in input.lines() {
            let mut parts = line.split([':', ',']);
            match parts.next() {
                Some("Button A") => {
                    button_a_x = parts.next().unwrap().split('+').last().unwrap().parse().unwrap();
                    button_a_y = parts.next().unwrap().split('+').last().unwrap().parse().unwrap();
                },
                Some("Button B") => {
                    button_b_x = parts.next().unwrap().split('+').last().unwrap().parse().unwrap();
                    button_b_y = parts.next().unwrap().split('+').last().unwrap().parse().unwrap();
                },
                Some("Prize") => {
                    let prize_x: i64 = parts.next().unwrap().split('=').last().unwrap().parse().unwrap();
                    let prize_y: i64 = parts.next().unwrap().split('=').last().unwrap().parse().unwrap();

                    // Part 1
                    let (a, b) = solve_linear_eq(button_a_x, button_b_x, prize_x, button_a_y, button_b_y, prize_y);
                    if a * button_a_x + b * button_b_x == prize_x && a * button_a_y + b * button_b_y == prize_y {
                        let tokens_a = 3 * a;
                        let tokens_b = b;
                        total_tokens_p1 += tokens_a + tokens_b;
                    }

                    // Part 2
                    let prize_x = prize_x + 10000000000000;
                    let prize_y = prize_y + 10000000000000;
                    let (a, b) = solve_linear_eq(button_a_x, button_b_x, prize_x, button_a_y, button_b_y, prize_y);
                    if a * button_a_x + b * button_b_x == prize_x && a * button_a_y + b * button_b_y == prize_y {
                        let tokens_a = 3 * a;
                        let tokens_b = b;
                        total_tokens_p2 += tokens_a + tokens_b;
                    }

                    (button_a_x, button_a_y, button_b_x, button_b_y) = (0, 0, 0, 0);
                }
                _ => {}
            }
        }

        (total_tokens_p1, total_tokens_p2)
    });
}