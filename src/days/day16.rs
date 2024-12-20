use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Display};
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

    pub fn adjacents(&self) -> Vec<Facing> {
        Facing::all()
            .into_iter()
            .filter(|f| *f != *self)
            .collect::<Vec<Facing>>()
    }

    pub fn opposite(&self) -> Facing {
        match self {
            Facing::East => Facing::West,
            Facing::South => Facing::North,
            Facing::West => Facing::East,
            Facing::North => Facing::South,
        }
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

fn dijkstra(maze: &Maze, cost_matrix: &mut Matrix<i64>, init_facing: Facing) -> HashMap<(Position, Facing), usize> {
    let mut visited = HashMap::new(); // (pos, facing) <=> (cost)
    let mut heap = BinaryHeap::new();

    // Push the starting position to the heap
    heap.push(State {
        position: maze.start,
        facing: init_facing,
        cost: 0,
    });

    while let Some(current) = heap.pop() {
        // If we have already visited this tile, and it is cheaper than the current cost, then skip it
        if let Some(visited_cost) = visited.get(&(current.position, current.facing)) {
            if *visited_cost <= current.cost {
                continue;
            }
        }

        // Otherwise mark the tile as visited
        visited.insert((current.position, current.facing), current.cost);

        // Update the cost matrix with this tile's new cost
        let current_cost = cost_matrix.get_mut(current.position.x, current.position.y).unwrap();
        if (current.cost as i64) <= *current_cost {
            *current_cost = current.cost as i64;
        }

        // If we've reached the end tile, stop the loop
        if current.position == maze.end {
            break;
        }

        // Check the next tile in all four directions except the opposite of the current
        for next_facing in current.facing.opposite().adjacents() {
            // Calculate the turn cost. If current.facing == next_facing then 1000, otherwise 0
            let turn_cost = next_facing.turn_cost(&current.facing);

            // Get the next position, skip if it's a wall
            let next_position = next_facing.apply(&current.position);
            if maze.is_wall(next_position.x, next_position.y) {
                continue;
            }

            // Next direction is a valid move, push it on the heap
            heap.push(State {
                position: next_position,
                facing: next_facing,
                cost: current.cost + 1 + turn_cost,
            });
        }
    }

    visited
}

fn trace_back(cost_matrix: &mut Matrix<i64>, position: &Position, unique_tiles: &mut HashSet<Position>, prev_cost: Option<i64>, visited: &HashMap<(Position, Facing), usize>) {
    // Store the position of the current tile
    unique_tiles.insert(*position);

    // Retrieve the cost of this tile
    let cost = *cost_matrix.get(position.x, position.y).unwrap();

    // Find an adjacent tile that is exactly 1 or 1001 cheaper
    for next_facing in Facing::all() {
        let next_position = next_facing.apply(&position);
        let next_cost = *cost_matrix.get(next_position.x, next_position.y).unwrap();

        // Skip if the next tile is a wall, or hasn't been touched by Dijkstra
        if next_cost == i64::MAX || next_cost == cost + 1 || next_cost == cost + 1001 {
            continue;
        }

        let prev_cost = prev_cost.unwrap_or(i64::MAX);

        let is_straight_case = next_cost == cost - 1;
        let is_corner_case = next_cost == cost - 1001;

        if is_straight_case || is_corner_case {
            trace_back(cost_matrix, &next_position, unique_tiles, Some(cost), visited);
        } else {
            // This is probably an intersection or a T-junction
            // First, check if there is a valid path straight ahead
            if let Some(&next_cost) = visited.get(&(*position, next_facing.opposite())) {
                if next_cost - 1000 == cost as usize && prev_cost - cost != 1 {
                    trace_back(cost_matrix, &next_position, unique_tiles, Some(cost), visited);
                }
            }

            for adj_facing in Facing::all() {
                if let Some(adj_cost) = visited.get(&(next_position, adj_facing)) {
                    if *adj_cost == cost as usize - 1 && next_facing == adj_facing.opposite() {
                        cost_matrix.set(next_position.x, next_position.y, *adj_cost as i64);
                        trace_back(cost_matrix, &next_position, unique_tiles, Some(*adj_cost as i64), visited);

                        break;
                    }
                }
            }
        }
    }
}

/// <b>Part 1:</b> Find the cheapest solution to the maze, accounting for the number of turns
/// <br/><br/><b>Part 2:</b> Count the number of unique tiles that are part of the best solutions to a maze
#[derive(Default)]
pub struct ReindeerMaze;

impl Solution for ReindeerMaze {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        // Load the maze from the input
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

        // Create a "Cost Matrix" to store the cost of each non-wall tile from the start tile (steps + 1000 * turns)
        let mut cost_matrix = maze.map.map(|_| i64::MAX);
        cost_matrix.set(maze.start.x, maze.start.y, 0);

        // Populate the cost matrix using Dijkstra's Algorithm to traverse the whole maze
        let visited = dijkstra(&maze, &mut cost_matrix, Facing::East);

        // By the end of dijkstra(), the end tile will store the cost of the cheapest path (the solution to Part 1)
        let lowest_cost = *cost_matrix.get(maze.end.x, maze.end.y).unwrap();

        // Recursively walk all "downwards slope" paths from end to start, storing each unique tile
        let mut unique_tiles = HashSet::new();
        trace_back(&mut cost_matrix, &maze.end, &mut unique_tiles, None, &visited);

        solution!(lowest_cost, unique_tiles.len())
    }
}