use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use crate::utils::matrix::Matrix;
use crate::utils::solution::{solution, Solution};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Facing {
    pub fn turn(self) -> Facing {
        match self {
            Facing::Up => Facing::Right,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::Right => Facing::Down,
        }
    }

    pub fn get_delta(self) -> (i32, i32) {
        match self {
            Facing::Up => (0, -1),
            Facing::Down => (0, 1),
            Facing::Left => (-1, 0),
            Facing::Right => (1, 0),
        }
    }
}

#[derive(Debug, Clone)]
struct Guard {
    pos_x: usize,
    pos_y: usize,
    facing: Facing,
}

#[derive(Debug, Clone)]
struct Room {
    obstacles: HashSet<(usize, usize)>,
    visited_tiles: HashMap<(usize, usize), Facing>,
    visited_tiles_and_facings: HashSet<(usize, usize, Facing)>,
    possible_new_obstacles: HashSet<(usize, usize)>,
    guard: Guard,
    width: usize,
    height: usize,
}

#[derive(Debug, Eq, PartialEq)]
enum RoomState {
    GuardPatrolling,
    GuardExited,
    GuardInLoop,
}

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.obstacles.contains(&(x, y)) {
                    str.push('█');
                } else if (x, y) == (self.guard.pos_x, self.guard.pos_y) {
                    str.push(match self.guard.facing {
                        Facing::Up => '^',
                        Facing::Down => 'v',
                        Facing::Left => '<',
                        Facing::Right => '>',
                    });
                } else if self.possible_new_obstacles.contains(&(x, y)) {
                    str.push('O');
                } else if let Some(facing) = self.visited_tiles.get(&(x, y)) {
                    str.push(match facing {
                        Facing::Up => '↑',
                        Facing::Down => '↓',
                        Facing::Left => '←',
                        Facing::Right => '→',
                    });
                } else {
                    str.push('.');
                }
            }
            str += "\n";
        }

        f.write_str(&str)
    }
}

fn process_room(room: &mut Room) -> RoomState {
    if let Some(_) = room.visited_tiles_and_facings.get(&(room.guard.pos_x, room.guard.pos_y, room.guard.facing)) {
        return RoomState::GuardInLoop;
    }
    room.visited_tiles_and_facings.insert((room.guard.pos_x, room.guard.pos_y, room.guard.facing));
    room.visited_tiles.insert((room.guard.pos_x, room.guard.pos_y), room.guard.facing);

    let guard_delta = room.guard.facing.get_delta();
    let guard_next_x = room.guard.pos_x as i32 + guard_delta.0;
    let guard_next_y = room.guard.pos_y as i32 + guard_delta.1;

    if guard_next_x < 0 || guard_next_y < 0 || guard_next_x >= room.width as i32 || guard_next_y >= room.height as i32 {
        // Guard has exited the room, return None
        RoomState::GuardExited
    } else if room.obstacles.contains(&(guard_next_x as usize, guard_next_y as usize)) {
        // Turn 90deg clockwise
        room.guard.facing = room.guard.facing.turn();
        RoomState::GuardPatrolling
    } else {
        // Step forward
        room.guard.pos_x = guard_next_x as usize;
        room.guard.pos_y = guard_next_y as usize;

        RoomState::GuardPatrolling
    }
}

#[derive(Default)]
pub struct GuardGallivant;

impl Solution for GuardGallivant {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let input_matrix = Matrix::<char>::from_text(input.as_str());

        // Part 1
        let mut room = Room {
            obstacles: HashSet::new(),
            visited_tiles: HashMap::new(),
            visited_tiles_and_facings: HashSet::new(),
            possible_new_obstacles: HashSet::new(),
            guard: Guard { facing: Facing::Up, pos_x: 0, pos_y: 0 },
            width: input_matrix.width(),
            height: input_matrix.height(),
        };

        for x in 0..room.width {
            for y in 0..room.height {
                match input_matrix.get(x, y) {
                    Some('#') => {
                        room.obstacles.insert((x, y));
                    }
                    Some('^') => {
                        room.guard.facing = Facing::Up;
                        room.guard.pos_x = x;
                        room.guard.pos_y = y;
                    }
                    Some('v') => {
                        room.guard.facing = Facing::Down;
                        room.guard.pos_x = x;
                        room.guard.pos_y = y;
                    }
                    Some('>') => {
                        room.guard.facing = Facing::Right;
                        room.guard.pos_x = x;
                        room.guard.pos_y = y;
                    }
                    Some('<') => {
                        room.guard.facing = Facing::Left;
                        room.guard.pos_x = x;
                        room.guard.pos_y = y;
                    }
                    _ => {}
                };
            }
        }

        let mut room_clone = room.clone();
        while let RoomState::GuardPatrolling = process_room(&mut room_clone) {
            continue;
        }

        // Part 2
        let mut possible_obstacle_count = 0;
        for (x, y) in room_clone.visited_tiles.keys() {
            let mut room_clone = room.clone();
            room_clone.obstacles.insert((*x, *y));
            loop {
                let state = process_room(&mut room_clone);
                if state == RoomState::GuardPatrolling {
                    continue;
                }

                if state == RoomState::GuardInLoop {
                    possible_obstacle_count += 1;
                }

                break;
            }
        }

        solution!(room_clone.visited_tiles.len(), possible_obstacle_count)
    }
}