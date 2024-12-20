use std::fmt::Display;
use rayon::prelude::*;
use ahash::{HashMap};
use crate::utils::facing::Facing;
use crate::utils::matrix::Matrix;
use crate::utils::position::Position;
use crate::utils::solution::{solution, Solution};

#[derive(Default)]
struct RaceTrack {
    map: Matrix<bool>,
    pos_distances: HashMap<Position, usize>,
    start: Position,
    end: Position,
}

impl RaceTrack {
    fn calculate_distances(&mut self) {
        let mut current_position = Some(self.end);
        let mut distance = 0;
        while let Some(position) = current_position.take() {
            self.pos_distances.insert(position, distance);
            for facing in Facing::all() {
                let next_position = facing.apply(&position);
                if self.pos_distances.contains_key(&next_position) {
                    continue;
                }

                if self.map.get(next_position.x, next_position.y).is_some_and(|m| *m) {
                    continue;
                }

                current_position.replace(next_position);
                distance += 1;
                break;
            }
        }
    }

    fn check_shortcuts_simple(&self, from_position: &Position, from_dist: usize) -> usize { // picoseconds => cheats
        let mut saved_min_100: usize = 0;

        for facing in Facing::all() {
            let next_position = facing.apply(&from_position);

            // If we're on the outer wall, stop processing
            if next_position.x == 0 || next_position.y == 0 || next_position.x == self.map.width() - 1 || next_position.y == self.map.height() - 1 {
                continue;
            }

            // If the next position is part of the track, stop processing
            if self.pos_distances.contains_key(&next_position) {
                continue;
            }

            let next_position_after = facing.apply(&next_position);
            let next_dist_after = self.pos_distances.get(&next_position_after);

            // If we're still in a wall, stop processing
            if next_dist_after.is_none() {
                continue;
            }

            let next_dist_after = *next_dist_after.unwrap();
            if next_dist_after >= from_dist {
                continue;
            }

            let saved_dist = from_dist - next_dist_after - 2;
            if saved_dist >= 100 {
                saved_min_100 += 1;
            }
        }

        saved_min_100
    }

    fn get_long_cheat_track(&self, from_position: &Position, from_cost: usize, cheat_dist: usize, min_saved: usize) -> usize {
        let mut new_track = Self {
            map: self.map.clone(),
            pos_distances: self.pos_distances.clone(),
            start: self.start.clone(),
            end: self.end.clone(),
        };

        let mut saved_min_100: usize = 0;

        let min_x = (from_position.x as i32 - cheat_dist as i32).max(1) as usize;
        let min_y = (from_position.y as i32 - cheat_dist as i32).max(1) as usize;
        let max_x = (from_position.x as i32 + cheat_dist as i32).min((new_track.map.width() - 2) as i32) as usize;
        let max_y = (from_position.y as i32 + cheat_dist as i32).min((new_track.map.width() - 2) as i32) as usize;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let dist_x = x.abs_diff(from_position.x);
                let dist_y = y.abs_diff(from_position.y);
                if dist_x + dist_y > cheat_dist {
                    continue;
                }

                let old_cost = new_track.pos_distances.remove(&Position::new(x, y)).unwrap_or(usize::MAX);
                if from_cost > old_cost {
                    let saved = from_cost - old_cost - (dist_x + dist_y);
                    if saved >= min_saved {
                        saved_min_100 += 1;
                    }
                }

                new_track.map.set(x, y, false);
            }
        }

        saved_min_100
    }
}

#[derive(Default)]
pub struct RaceCondition;

impl Solution for RaceCondition {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let mut track = RaceTrack::default();
        let map = Matrix::<char>::from_text(&input)
            .map_xy(|data, x, y| {
                if *data == 'S' {
                    track.start = Position { x, y };
                } else if *data == 'E' {
                    track.end = Position { x, y };
                }
                *data == '#'
            });
        track.map = map;

        track.calculate_distances();

        // Part 1
        let total_two_cheats = track.pos_distances.par_iter()
            .map(|(pos, dist)| track.check_shortcuts_simple(pos, *dist))
            .sum::<usize>();

        // Part 2
        let total_twenty_cheats = track.pos_distances.par_iter()
            .map(|(pos, cost)| track.get_long_cheat_track(pos, *cost, 20, 100))
            .sum::<usize>();

        let mut sorted_pos_distances: Vec<(Position, usize)> = track.pos_distances.clone().into_iter().collect();
        sorted_pos_distances.sort_by(|a, b| b.1.cmp(&a.1));

        solution!(total_two_cheats, total_twenty_cheats)
    }
}