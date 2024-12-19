use std::fmt::Display;
use crate::utils::matrix::Matrix;
use crate::utils::solution::{solution, Solution};

fn check_star(matrix: &Matrix<char>, word: &str, x: u32, y: u32, dx: i8, dy: i8) -> u8 {
    let len = word.len();
    let end_x: i32 = x as i32 + (dx as i32 * (len - 1) as i32);
    let end_y: i32 = y as i32 + (dy as i32 * (len - 1) as i32);

    if end_x < 0 || end_y < 0 || end_x > matrix.width() as i32 || end_y > matrix.height() as i32 {
        return 0;
    }

    for (i, c) in word.char_indices() {
        let char_x = (x as i32 + (dx as i32 * i as i32)) as usize;
        let char_y = (y as i32 + (dy as i32 * i as i32)) as usize;
        if char_x >= matrix.width() || char_y >= matrix.height() {
            return 0
        }
        let mat_char = *matrix.get(char_x, char_y).unwrap();
        if mat_char != c {
            return 0
        }
    }

    // println!("Found at {:?} -> {:?}", (x, y), (dx, dy));
    1
}

fn try_count_xmas_star(matrix: &Matrix<char>, word: &str, x: u32, y: u32) -> u8 {
    let mut count = 0;
    count += check_star(matrix, word, x, y, 1, 0); // Right
    count += check_star(matrix, word, x, y, -1, 0); // Left
    count += check_star(matrix, word, x, y, 0, 1); // Down
    count += check_star(matrix, word, x, y, 0, -1); // Up
    count += check_star(matrix, word, x, y, 1, 1); // LowerRight
    count += check_star(matrix, word, x, y, 1, -1); // UpperRight
    count += check_star(matrix, word, x, y, -1, 1); // LowerLeft
    count += check_star(matrix, word, x, y, -1, -1); // UpperLeft

    count
}

fn try_find_x_mas(matrix: &Matrix<char>, x: u32, y: u32) -> bool {
    if x <= 0 || y <= 0 || x + 1 >= matrix.width() as u32 || y + 1 == matrix.height() as u32 {
        false
    } else {
        let ch = *matrix.get(x as usize, y as usize).unwrap();
        if ch != 'A' {
            false
        } else {
            let top_left = *matrix.get((x - 1) as usize, (y - 1) as usize).unwrap();
            let bottom_left = *matrix.get((x - 1) as usize, (y + 1) as usize).unwrap();
            let top_right = *matrix.get((x + 1) as usize, (y - 1) as usize).unwrap();
            let bottom_right = *matrix.get((x + 1) as usize, (y + 1) as usize).unwrap();

            let diag_1 = top_left == 'M' && bottom_right == 'S' || top_left == 'S' && bottom_right == 'M';
            let diag_2 = bottom_left == 'M' && top_right == 'S' || bottom_left == 'S' && top_right == 'M';

            diag_1 && diag_2
        }
    }
}

/// <b>Part 1:</b> Count the occurrences of "XMAS" in a crossword puzzle
/// <br/><br/><b>Part 2:</b> Count the occurrences of X shaped "MAS" patterns
#[derive(Default)]
pub struct CeresSearch;

impl Solution for CeresSearch {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let matrix = Matrix::<char>::from_text(&input).map(|c| c.to_ascii_uppercase());

        let mut total: u32 = 0;
        let mut total_cross: u32 = 0;
        matrix.each(|x, y, _| {
            total += try_count_xmas_star(&matrix, "XMAS", *x as u32, *y as u32) as u32;

            if try_find_x_mas(&matrix, *x as u32, *y as u32) {
                total_cross += 1
            }
        });

        solution!(total_cross, total)
    }
}