use crate::utils::matrix::Matrix;
use crate::utils::solution::{solution, Solution};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
enum MapTile {
    Wall,
    Empty,
    Box,

    BoxLeft,
    BoxRight,
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Clone)]
struct Warehouse {
    map: Matrix<MapTile>,
    robot_x: usize,
    robot_y: usize,
}

impl Display for Warehouse {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.map.height() {
            for x in 0..self.map.width() {
                if (self.robot_x, self.robot_y) == (x, y) {
                    print!("@");
                } else {
                    let tile = self.map.get(x, y).unwrap();
                    print!("{}", match tile {
                        MapTile::Wall => "#",
                        MapTile::Empty => ".",
                        MapTile::Box => "O",
                        MapTile::BoxLeft => "[",
                        MapTile::BoxRight => "]",
                    });
                }
            }
            println!();
        }

        Ok(())
    }
}

/// Move box at (from_x, from_y) in direction
fn shift_boxes(warehouse: &mut Warehouse, from_x: i32, from_y: i32, direction: &Direction) -> bool {
    if from_x <= 0 || from_y <= 0 || from_x >= warehouse.map.width() as i32 || from_y >= warehouse.map.height() as i32 {
        return false;
    }

    let tile = warehouse.map.get(from_x as usize, from_y as usize).unwrap().clone();
    if let MapTile::Wall = tile {
        return false;
    } else if let MapTile::Empty = tile {
        return true;
    }

    let (delta_x, delta_y) = direction.delta();
    let next_box_x = from_x + delta_x;
    let next_box_y = from_y + delta_y;

    if !shift_boxes(warehouse, next_box_x, next_box_y, direction) {
        return false;
    }

    *warehouse.map.get_mut(next_box_x as usize, next_box_y as usize).unwrap() = MapTile::Box;
    *warehouse.map.get_mut(from_x as usize, from_y as usize).unwrap() = MapTile::Empty;

    true
}

fn simulate_robot_step(warehouse: &mut Warehouse, direction: &Direction) {
    let (delta_x, delta_y) = direction.delta();
    if shift_boxes(warehouse, warehouse.robot_x as i32 + delta_x, warehouse.robot_y as i32 + delta_y, &direction) {
        warehouse.robot_x = (warehouse.robot_x as i32 + delta_x) as usize;
        warehouse.robot_y = (warehouse.robot_y as i32 + delta_y) as usize;
    }
}

fn test_shift_boxes_wide(warehouse: &mut Warehouse, from_x: i32, from_y: i32, direction: &Direction) -> bool {
    if from_x <= 1 || from_y <= 0 || from_x >= warehouse.map.width() as i32 - 1 || from_y >= warehouse.map.height() as i32 {
        return false;
    }

    let tile = warehouse.map.get(from_x as usize, from_y as usize).unwrap().clone();
    if let MapTile::Wall = tile {
        return false;
    } else if let MapTile::Empty = tile {
        return true;
    }

    let (delta_x, delta_y) = direction.delta();
    let next_box_x = from_x + delta_x;
    let next_box_y = from_y + delta_y;

    if let Direction::Left | Direction::Right = direction {
        test_shift_boxes_wide(warehouse, next_box_x, next_box_y, direction)
    } else {
        let x_delta = match tile {
            MapTile::BoxLeft => 1,
            MapTile::BoxRight => -1,
            _ => panic!("This should not happen"),
        };
        let branch_1 = test_shift_boxes_wide(warehouse, next_box_x, next_box_y, direction);
        let branch_2 = test_shift_boxes_wide(warehouse, next_box_x + x_delta, next_box_y, direction);

        branch_1 && branch_2
    }
}

fn do_shift_boxes_wide(warehouse: &mut Warehouse, from_x: i32, from_y: i32, direction: &Direction) {
    if from_x <= 1 || from_y <= 0 || from_x >= warehouse.map.width() as i32 - 1 || from_y >= warehouse.map.height() as i32 {
        return;
    }

    let tile = warehouse.map.get(from_x as usize, from_y as usize).unwrap().clone();
    if let MapTile::Wall | MapTile::Empty = tile {
        return;
    }

    let (delta_x, delta_y) = direction.delta();
    let next_box_x = from_x + delta_x;
    let next_box_y = from_y + delta_y;

    let x_delta = match tile {
        MapTile::BoxLeft => 1,
        MapTile::BoxRight => -1,
        _ => panic!("This should not happen"),
    };
    do_shift_boxes_wide(warehouse, next_box_x, next_box_y, direction);
    if let Direction::Up | Direction::Down = direction {
        do_shift_boxes_wide(warehouse, next_box_x + x_delta, next_box_y, direction);
    }

    *warehouse.map.get_mut(next_box_x as usize, next_box_y as usize).unwrap() = tile.clone();
    *warehouse.map.get_mut(from_x as usize, from_y as usize).unwrap() = MapTile::Empty;

    if let Direction::Up | Direction::Down = direction {
        *warehouse.map.get_mut((next_box_x + x_delta) as usize, next_box_y as usize).unwrap() = match tile {
            MapTile::BoxLeft => MapTile::BoxRight,
            MapTile::BoxRight => MapTile::BoxLeft,
            _ => panic!("This should not happen #2"),
        };
        *warehouse.map.get_mut((from_x + x_delta) as usize, from_y as usize).unwrap() = MapTile::Empty;
    }
}

fn simulate_robot_step_wide(warehouse: &mut Warehouse, direction: &Direction) {
    let (delta_x, delta_y) = direction.delta();
    if test_shift_boxes_wide(warehouse, warehouse.robot_x as i32 + delta_x, warehouse.robot_y as i32 + delta_y, &direction) {
        do_shift_boxes_wide(warehouse, warehouse.robot_x as i32 + delta_x, warehouse.robot_y as i32 + delta_y, &direction);
        warehouse.robot_x = (warehouse.robot_x as i32 + delta_x) as usize;
        warehouse.robot_y = (warehouse.robot_y as i32 + delta_y) as usize;
    }
}

/// <b>Part 1:</b> Simulate a robot moving boxes in a room
/// <br/><br/><b>Part 2:</b> Simulate the same room, with double-wide boxes
#[derive(Default)]
pub struct WarehouseWoes;

impl Solution for WarehouseWoes {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let mut input_matrix = String::new();
        let mut input_directions = String::new();
        let mut reading_directions = false;
        for line in input.lines() {
            if line.is_empty() {
                reading_directions = true;
            }

            if !reading_directions {
                input_matrix += &*(line.to_string() + "\n");
            } else {
                input_directions += line;
            }
        }

        let mut robot_x = 0;
        let mut robot_y = 0;

        let map = Matrix::<char>::from_text(&input_matrix)
            .map_xy(|ch, x, y| {
                match ch {
                    '#' => MapTile::Wall,
                    '.' => MapTile::Empty,
                    'O' => MapTile::Box,
                    '@' => {
                        (robot_x, robot_y) = (x, y);
                        MapTile::Empty
                    }
                    _ => panic!("Invalid character in input map data!")
                }
            });

        let directions = input_directions.chars()
            .map(|ch| {
                match ch {
                    '>' => Direction::Right,
                    '<' => Direction::Left,
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    _ => panic!("Invalid character in input directions data!")
                }
            })
            .collect::<Vec<_>>();

        let mut warehouse = Warehouse { map: map.clone(), robot_x, robot_y };

        // Part 1
        for direction in &directions {
            simulate_robot_step(&mut warehouse, direction);
        }

        let mut gps_sum = 0;
        for y in 0..warehouse.map.height() {
            for x in 0..warehouse.map.width() {
                let tile = warehouse.map.get(x, y).unwrap();
                if let MapTile::Box = tile {
                    gps_sum += x + 100 * y;
                }
            }
        }

        // Part 2
        let mut wide_map: Matrix<MapTile> = Matrix::new_fill(map.width() * 2, map.height(), MapTile::Empty);
        for y in 0..map.height() {
            for x in 0..map.width() {
                let tile = map.get(x, y).unwrap();
                match tile {
                    MapTile::Box => {
                        wide_map.set(x * 2, y, MapTile::BoxLeft);
                        wide_map.set(x * 2 + 1, y, MapTile::BoxRight);
                    }
                    tile => {
                        wide_map.set(x * 2, y, tile.clone());
                        wide_map.set(x * 2 + 1, y, tile.clone());
                    }
                }
            }
        }

        let mut warehouse = Warehouse { map: wide_map, robot_x: robot_x * 2, robot_y };
        for direction in &directions {
            simulate_robot_step_wide(&mut warehouse, direction);
        }

        let mut gps_sum_wide = 0;
        for y in 0..warehouse.map.height() {
            for x in 0..warehouse.map.width() {
                let tile = warehouse.map.get(x, y).unwrap();
                if let MapTile::BoxLeft = tile {
                    gps_sum_wide += x + 100 * y;
                }
            }
        }


        solution!(gps_sum, gps_sum_wide)
    }
}