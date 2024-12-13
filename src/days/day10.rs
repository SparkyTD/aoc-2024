use std::collections::HashSet;
use std::fmt::Display;
use crate::utils::matrix::Matrix;
use crate::utils::solution::{solution, Solution};

fn get_destination_count(map: &Matrix<u8>, start_x: i16, start_y: i16, prev_value: Option<u8>) -> Option<HashSet<(i16, i16)>> {
    if start_x < 0 || start_x >= map.width() as i16 || start_y < 0 || start_y >= map.height() as i16 {
        return None;
    }

    let current_value = *map.get(start_x as usize, start_y as usize).unwrap();
    if let Some(prev_value) = prev_value {
        if current_value != prev_value + 1 {
            return None;
        }
    }

    if current_value == 9 {
        let mut hash_set = HashSet::new();
        hash_set.insert((start_x, start_y));
        return Some(hash_set);
    }

    let mut hash_set = HashSet::new();
    let up = get_destination_count(map, start_x, start_y - 1, Some(current_value));
    let down = get_destination_count(map, start_x, start_y + 1, Some(current_value));
    let left = get_destination_count(map, start_x - 1, start_y, Some(current_value));
    let right = get_destination_count(map, start_x + 1, start_y, Some(current_value));

    if let Some(up) = up {
        hash_set.extend(up);
    }
    if let Some(down) = down {
        hash_set.extend(down);
    }
    if let Some(left) = left {
        hash_set.extend(left);
    }
    if let Some(right) = right {
        hash_set.extend(right);
    }

    Some(hash_set)
}

fn get_unique_trail_count(map: &Matrix<u8>, start_x: i16, start_y: i16, prev_value: Option<u8>) -> u8 {
    if start_x < 0 || start_x >= map.width() as i16 || start_y < 0 || start_y >= map.height() as i16 {
        return 0;
    }

    let current_value = *map.get(start_x as usize, start_y as usize).unwrap();
    if let Some(prev_value) = prev_value {
        if current_value != prev_value + 1 {
            return 0;
        }
    }

    if current_value == 9 {
        return 1;
    }

    let up = get_unique_trail_count(map, start_x, start_y - 1, Some(current_value));
    let down = get_unique_trail_count(map, start_x, start_y + 1, Some(current_value));
    let left = get_unique_trail_count(map, start_x - 1, start_y, Some(current_value));
    let right = get_unique_trail_count(map, start_x + 1, start_y, Some(current_value));

    up + down + left + right
}

#[derive(Default)]
pub struct HoofIt;

impl Solution for HoofIt {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let map: Matrix<u8> = Matrix::<char>::from_text(input.as_str())
            .map(|c| c.to_digit(10).unwrap() as u8);
        let mut trailheads: Vec<(u8, u8)> = Vec::new(); // (x, y)
        map.each(|x, y, height| {
            if *height == 0 {
                trailheads.push((*x as u8, *y as u8))
            }
        });

        let score = trailheads.iter()
            .map(|&t| get_destination_count(&map, t.0 as i16, t.1 as i16, None).map_or(0, |h| h.len()))
            .sum::<usize>();

        let rating = trailheads.iter()
            .map(|&t| get_unique_trail_count(&map, t.0 as i16, t.1 as i16, None) as usize)
            .sum::<usize>();

        solution!(score, rating)
    }
}