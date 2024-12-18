use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};
use std::hash::{Hash};
use crate::utils::matrix::Matrix;
use crate::utils::solution::{solution, Solution};

#[derive(Default)]
struct Maze {
    map: Matrix<bool>,
    start: Position,
    end: Position,
}

impl Maze {
    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        *self.map.get(x, y).unwrap_or(&false)
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.map.height() {
            for x in 0..self.map.width() {
                if (x, y) == (self.start.x, self.start.y) {
                    f.write_str("S")?;
                } else if (x, y) == (self.end.x, self.end.y) {
                    f.write_str("E")?;
                } else {
                    let tile = self.map.get(x, y).unwrap();
                    f.write_str(if *tile { "#" } else { "." })?;
                }
            }
            f.write_char('\n')?
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Facing {
    East,
    South,
    West,
    North,
}

impl Facing {
    pub fn apply(&self, position: &Position) -> Position {
        match self {
            Facing::East => Position { x: position.x + 1, y: position.y },
            Facing::West => Position { x: position.x - 1, y: position.y },
            Facing::South => Position { x: position.x, y: position.y + 1 },
            Facing::North => Position { x: position.x, y: position.y - 1 },
        }
    }

    fn turn_cost(&self, other: &Facing) -> usize {
        if self == other { 0 } else { 1000 }
    }

    pub fn all() -> Vec<Facing> {
        vec![Facing::North, Facing::East, Facing::South, Facing::West]
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct State {
    position: Position,
    facing: Facing,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse for min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

fn dijkstra(maze: &Maze, cost_matrix: &mut Matrix<i64>, init_facing: Facing) {
    let mut visited = HashMap::new(); // (pos, facing) <=> (cost)
    let mut heap = BinaryHeap::new();

    heap.push(State {
        position: maze.start,
        facing: init_facing,
        cost: 0,
    });

    while let Some(current) = heap.pop() {
        if let Some(visited_cost) = visited.get(&(current.position, current.facing)) {
            if *visited_cost <= current.cost {
                continue;
            }
        }

        visited.insert((current.position, current.facing), current.cost);

        let current_cost = cost_matrix.get_mut(current.position.x, current.position.y).unwrap();
        if (current.cost as i64) < *current_cost {
            // println!("[{:?}] {} <- {}", current.position, *current_cost, current.cost);
            *current_cost = current.cost as i64;
        }

        let next_position = current.facing.apply(&current.position);
        if !maze.is_wall(next_position.x, next_position.y) {
            heap.push(State {
                position: next_position,
                facing: current.facing,
                cost: current.cost + 1,
            });
        }

        for next_facing in Facing::all() {
            if next_facing == current.facing {
                continue;
            }

            let next_position = next_facing.apply(&current.position);
            if maze.is_wall(next_position.x, next_position.y) {
                continue;
            }

            heap.push(State {
                position: next_position,
                facing: next_facing,
                cost: current.cost + 1 + current.facing.turn_cost(&next_facing),
            });
        }
    }
}

fn find_all_unique_tiles(cost_matrix: &mut Matrix<i64>, position: &Position, unique_tiles: &mut HashSet<Position>, prev_cost: Option<i64>, shortcuts: &mut HashSet<i64>, shortcut_count: &mut usize) {
    unique_tiles.insert(*position);

    let cost = *cost_matrix.get(position.x, position.y).unwrap();

    for facing in Facing::all() {
        let next_position = facing.apply(&position);
        let next_cost = *cost_matrix.get(next_position.x, next_position.y).unwrap();
        if next_cost == i64::MAX {
            continue;
        }

        if next_cost < cost {
            find_all_unique_tiles(cost_matrix, &next_position, unique_tiles, Some(cost), shortcuts, shortcut_count);
        } else if let Some(prev_cost) = prev_cost {
            if cost == prev_cost - 1000 - 1 && prev_cost - next_cost == 2 {
                if shortcuts.contains(&prev_cost) {
                    *shortcut_count += 1;
                }
                shortcuts.insert(next_cost);
                cost_matrix.set(position.x, position.y, i64::MAX);
                find_all_unique_tiles(cost_matrix, &next_position, unique_tiles, Some(next_cost), shortcuts, shortcut_count);
            }
        }
    }
}

#[derive(Default)]
pub struct ReindeerMaze;

impl Solution for ReindeerMaze {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let mut maze = Maze::default();
        let map = Matrix::<char>::from_text(&input)
            .map_xy(|data, x, y| {
                if *data == 'S' {
                    maze.start = Position { x, y };
                } else if *data == 'E' {
                    maze.end = Position { x, y };
                }
                *data == '#'
            });
        maze.map = map;

        let mut cost_matrix = maze.map.map(|_| i64::MAX);
        cost_matrix.set(maze.start.x, maze.start.y, 0);

        dijkstra(&maze, &mut cost_matrix, Facing::East);

        let lowest_cost = *cost_matrix.get(maze.end.x, maze.end.y).unwrap();

        let mut unique_tiles = HashSet::new();
        let mut shortcuts = HashSet::new();
        let mut shortcut_count = 0;
        find_all_unique_tiles(&mut cost_matrix, &maze.end, &mut unique_tiles, None, &mut shortcuts, &mut shortcut_count);
        solution!(lowest_cost, unique_tiles.len() - shortcut_count)
    }
}