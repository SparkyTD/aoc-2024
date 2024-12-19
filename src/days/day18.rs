use std::collections::{HashSet, VecDeque};
use std::fmt::Display;
use crate::utils::matrix::Matrix;
use crate::utils::solution::{solution, Solution};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct Area {
    map: Matrix<bool>,
    start: Position,
    end: Position,
    byte_queue: VecDeque<Position>,
}

impl Area {
    pub fn drop_bytes(&mut self, mut count: u32) {
        while count > 0 {
            let byte = self.byte_queue.pop_front().unwrap();
            self.map.set(byte.x as usize, byte.y as usize, true);
            count -= 1;
        }
    }

    pub fn drop_one_byte(&mut self) -> Position {
        let byte = self.byte_queue.pop_front().unwrap();
        self.map.set(byte.x as usize, byte.y as usize, true);
        byte
    }

    pub fn shortest_path_length(&self) -> HashSet<Position> {
        let start = self.start;
        let end = self.end;

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent_matrix = Matrix::new_fill(self.map.width(), self.map.height(), None);

        queue.push_back(start);

        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        while let Some(position) = queue.pop_front() {
            if position == end {
                let mut current = position;
                let mut path_parts = HashSet::new();
                while let Some(Some(position)) = parent_matrix.get(current.x as usize, current.y as usize) {
                    current = *position;
                    path_parts.insert(current);

                    if current == start {
                        break;
                    }
                }
                return path_parts;
            }

            for delta in &directions {
                let next_position = Position::new(position.x + delta.0, position.y + delta.1);
                if next_position.x < 0 || next_position.y < 0 || next_position.x >= self.map.width() as i32 || next_position.y >= self.map.height() as i32 {
                    continue;
                }

                if visited.contains(&next_position) {
                    continue;
                }
                visited.insert(next_position);

                let tile = self.map.get(next_position.x as usize, next_position.y as usize).unwrap();
                if *tile {
                    continue;
                }

                queue.push_back(next_position);
                parent_matrix.set(next_position.x as usize, next_position.y as usize, Some(position));
            }
        }

        HashSet::new()
    }
}

/// <b>Part 1:</b> Simulate 1024 falling bytes in a 2D grid, find the shortest path to exit
/// <br/><br/><b>Part 2:</b> Find the number of fallen bytes before the exit becomes unreachable
#[derive(Default)]
pub struct RAMRun;

impl Solution for RAMRun {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let byte_positions = input
            .lines()
            .map(|l| l.split(',').collect::<Vec<&str>>())
            .map(|s| Position::new(s[0].parse::<i32>().unwrap(), s[1].parse::<i32>().unwrap()))
            .collect::<Vec<Position>>();

        let max_position_x = byte_positions.iter().max_by_key(|p| p.x).unwrap().x as usize;
        let max_position_y = byte_positions.iter().max_by_key(|p| p.y).unwrap().y as usize;

        let mut area = Area {
            map: Matrix::<bool>::new(max_position_x + 1, max_position_y + 1),
            start: Position::new(0, 0),
            end: Position::new(max_position_x as i32, max_position_y as i32),
            byte_queue: VecDeque::from(byte_positions.clone()),
        };

        // Part 1
        let bytes = if max_position_x > 50 { 1024 } else { 12 };
        area.drop_bytes(bytes);

        let length = area.shortest_path_length();

        // Part 2
        let mut last_path = HashSet::new();
        area.byte_queue = VecDeque::from(byte_positions);
        let mut last_byte_dropped = None;
        while !area.byte_queue.is_empty() {
            let next_byte_to_drop = area.drop_one_byte();
            last_byte_dropped = Some(next_byte_to_drop);
            if !last_path.is_empty() && !last_path.contains(&next_byte_to_drop) {
                continue;
            }

            last_path = area.shortest_path_length();
            if last_path.len() == 0 {
                break;
            }
        }

        let last_byte_dropped = last_byte_dropped
            .and_then(|p| Some(format!("{},{}", p.x, p.y)))
            .unwrap_or("(none)".to_string());

        solution!(length.len(), last_byte_dropped)
    }
}