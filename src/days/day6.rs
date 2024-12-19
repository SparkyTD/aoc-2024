use std::collections::{HashSet};
use std::fmt::Display;
use ahash::AHashSet;
use rayon::prelude::*;
use crate::utils::matrix::Matrix;
use crate::utils::solution::{solution, Solution};

//type MyHashSet<T> = HashSet<T>; // 27s 261ms
type MyHashSet<T> = AHashSet<T>; // 9s 690ms
//type MyHashSet<T> = FxHashSet<T>; // 17s 201ms

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
    pos_x: u8,
    pos_y: u8,
    facing: Facing,
}

#[derive(Debug, Clone)]
struct Room {
    obstacles: HashSet<(u8, u8)>,
    visited_tiles_and_facings: MyHashSet<u32>,
    possible_new_obstacles: HashSet<(u8, u8)>,
    guard: Guard,
    width: u8,
    height: u8,
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
        for y in 0..self.height as u8 {
            for x in 0..self.width as u8 {
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
                } else {
                    str.push('.');
                }
            }
            str += "\n";
        }

        f.write_str(&str)
    }
}

#[inline]
fn pack_numbers(a: u8, b: u8, c: u8) -> u32 {
    ((a as u32) << 16) | ((b as u32) << 8) | (c as u32)
}

#[inline]
fn unpack_numbers(packed: u32) -> (u8, u8, u8) {
    let a = ((packed >> 16) & 0xFF) as u8;
    let b = ((packed >> 8) & 0xFF) as u8;
    let c = (packed & 0xFF) as u8;
    (a, b, c)
}

fn process_room(room: &mut Room) -> RoomState {
    if !room.visited_tiles_and_facings.insert(pack_numbers(room.guard.pos_x, room.guard.pos_y, room.guard.facing as u8)) {
        return RoomState::GuardInLoop;
    }

    let guard_delta = room.guard.facing.get_delta();
    let guard_next_x = room.guard.pos_x as i32 + guard_delta.0;
    let guard_next_y = room.guard.pos_y as i32 + guard_delta.1;

    if guard_next_x < 0 || guard_next_y < 0 || guard_next_x >= room.width as i32 || guard_next_y >= room.height as i32 {
        // Guard has exited the room, return None
        RoomState::GuardExited
    } else if room.obstacles.contains(&(guard_next_x as u8, guard_next_y as u8)) {
        // Turn 90deg clockwise
        room.guard.facing = room.guard.facing.turn();
        RoomState::GuardPatrolling
    } else {
        // Step forward
        room.guard.pos_x = guard_next_x as u8;
        room.guard.pos_y = guard_next_y as u8;

        RoomState::GuardPatrolling
    }
}

/// <b>Part 1:</b> Count how many paths are visited by the guard before they exit
/// <br/><br/><b>Part 2:</b> Count how many additional obstacles can be added to trap the guard in a loop
#[derive(Default)]
pub struct GuardGallivant;

impl Solution for GuardGallivant {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let input_matrix = Matrix::<char>::from_text(input.as_str());

        // Part 1
        let mut room = Room {
            obstacles: HashSet::new(),
            visited_tiles_and_facings: MyHashSet::new(),
            possible_new_obstacles: HashSet::new(),
            guard: Guard { facing: Facing::Up, pos_x: 0, pos_y: 0 },
            width: input_matrix.width() as u8,
            height: input_matrix.height() as u8,
        };

        for x in 0..room.width {
            for y in 0..room.height {
                match input_matrix.get(x as usize, y as usize) {
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

        let visited_tiles = room_clone.visited_tiles_and_facings
            .iter()
            .map(|val| unpack_numbers(*val))
            .map(|(x, y, _)| (x, y))
            .collect::<HashSet<_>>();

        // Part 2
        let mut parallel_input = visited_tiles
            .iter()
            .map(|&(x, y)| {
                let mut room_clone = room.clone();
                room_clone.obstacles.insert((x, y));

                room_clone
            }).collect::<Vec<_>>();
        let parallel_output = parallel_input
            .par_iter_mut()
            .map(|room| {
                let mut stuck_in_loop = false;
                loop {
                    let state = process_room(room);
                    if state == RoomState::GuardPatrolling {
                        continue;
                    }

                    if state == RoomState::GuardInLoop {
                        stuck_in_loop = true;
                    }

                    break;
                }
                stuck_in_loop
            })
            .filter(|b| *b)
            .count();

        solution!(visited_tiles.len(), parallel_output)
    }
}