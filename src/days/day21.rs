use std::fmt::Display;
use std::hash::Hash;
use ahash::HashMap;
use ahash::HashMapExt;
use indexmap::IndexMap;
use crate::utils::position::Position;
use crate::utils::solution::{solution, Solution};

#[derive(Default)]
pub struct KeypadConundrum;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum RobotAction {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum NumpadKey {
    Number1,
    Number2,
    Number3,
    Number4,
    Number5,
    Number6,
    Number7,
    Number8,
    Number9,
    Number0,
    Activate,
    Blank,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum DirectionalKey {
    Up,
    Down,
    Left,
    Right,
    Activate,
    Blank,
}

trait KeypadKey {
    fn is_blank(&self) -> bool;
    fn get_pad_size() -> (usize, usize);
    fn get_blank_position() -> Position;
}

impl KeypadKey for NumpadKey {
    fn is_blank(&self) -> bool {
        self == &NumpadKey::Blank
    }

    fn get_pad_size() -> (usize, usize) {
        (3, 4)
    }

    fn get_blank_position() -> Position {
        Position::new(0, 3)
    }
}

impl KeypadKey for DirectionalKey {
    fn is_blank(&self) -> bool {
        self == &DirectionalKey::Blank
    }

    fn get_pad_size() -> (usize, usize) {
        (3, 2)
    }

    fn get_blank_position() -> Position {
        Position::new(0, 0)
    }
}

impl NumpadKey {
    fn from_char(c: char) -> Option<NumpadKey> {
        match c {
            '0' => Some(NumpadKey::Number0),
            '1' => Some(NumpadKey::Number1),
            '2' => Some(NumpadKey::Number2),
            '3' => Some(NumpadKey::Number3),
            '4' => Some(NumpadKey::Number4),
            '5' => Some(NumpadKey::Number5),
            '6' => Some(NumpadKey::Number6),
            '7' => Some(NumpadKey::Number7),
            '8' => Some(NumpadKey::Number8),
            '9' => Some(NumpadKey::Number9),
            'A' => Some(NumpadKey::Activate),
            _ => None,
        }
    }
}

impl DirectionalKey {
    fn from_robot_action(robot_action: RobotAction) -> DirectionalKey {
        match robot_action {
            RobotAction::Up => DirectionalKey::Up,
            RobotAction::Down => DirectionalKey::Down,
            RobotAction::Left => DirectionalKey::Left,
            RobotAction::Right => DirectionalKey::Right,
            RobotAction::Activate => DirectionalKey::Activate,
        }
    }
}

type PrecomputeMap<T> = HashMap<(T, T), Vec<RobotAction>>;

fn code_to_number(code: &str) -> usize {
    let mut num: usize = 0;
    for ch in code.chars() {
        if let Some(digit) = ch.to_digit(10) {
            num = num * 10 + digit as usize;
        }
    }
    num
}

fn precompute_best_numpad_moves<T>(numpad: &HashMap<Position, T>) -> HashMap<(T, T), Vec<RobotAction>>
where
    T: KeypadKey + Copy + Eq + PartialEq + Hash,
{
    let mut map = HashMap::new();

    let (width, height) = T::get_pad_size();
    let blank_position = T::get_blank_position();

    for x1 in 0..width {
        for y1 in 0..height {
            let btn1 = &numpad[&Position::new(x1, y1)];

            for x2 in 0..width {
                for y2 in 0..height {
                    let btn2 = &numpad[&Position::new(x2, y2)];

                    if btn1.is_blank() || btn2.is_blank() {
                        continue;
                    }

                    if *btn1 == *btn2 {
                        map.insert((*btn1, *btn2), vec![RobotAction::Activate]);
                        continue;
                    }

                    let delta_h = x2 as isize - x1 as isize;
                    let delta_v = y2 as isize - y1 as isize;

                    let cmd_h = (0..delta_h.abs()).map(|_| if delta_h > 0 { RobotAction::Right } else { RobotAction::Left }).collect::<Vec<_>>();
                    let cmd_v = (0..delta_v.abs()).map(|_| if delta_v > 0 { RobotAction::Down } else { RobotAction::Up }).collect::<Vec<_>>();

                    let mut cmd_full = if delta_h < 0 { cmd_h.iter().chain(&cmd_v).cloned() } else { cmd_v.iter().chain(&cmd_h).cloned() }.collect::<Vec<_>>();
                    if (x1 == blank_position.x || x2 == blank_position.x) && (y1 == blank_position.y || y2 == blank_position.y) {
                        cmd_full.reverse();
                    }
                    cmd_full.push(RobotAction::Activate);
                    map.insert((*btn1, *btn2), cmd_full);
                }
            }
        }
    }

    map
}

fn solve_numpad_movements(code: &Vec<NumpadKey>, numpad_precomp: &PrecomputeMap<NumpadKey>) -> IndexMap<(RobotAction, RobotAction), usize> {
    let mut moves_with_count = IndexMap::new();
    for i in 0..code.len() - 1 {
        let instruction = numpad_precomp.get(&(code[i], code[i + 1])).unwrap();
        let action = (RobotAction::Activate, instruction[0]);

        if let None = moves_with_count.get(&action) {
            moves_with_count.insert(action, 1);
        } else if let Some(action) = moves_with_count.get_mut(&action) {
            *action += 1;
        }

        for j in 0..instruction.len() - 1 {
            let action = (instruction[j], instruction[j + 1]);

            if let None = moves_with_count.get(&action) {
                moves_with_count.insert(action, 1);
            } else if let Some(action) = moves_with_count.get_mut(&action) {
                *action += 1;
            }
        }
    }
    moves_with_count
}

fn solve_direction_movements(moves_with_count: IndexMap<(RobotAction, RobotAction), usize>, direction_precomp: &PrecomputeMap<DirectionalKey>) -> IndexMap<(RobotAction, RobotAction), usize> {
    let mut new_moves_with_count = IndexMap::new();

    for (movement, count) in moves_with_count {
        let movement = (DirectionalKey::from_robot_action(movement.0), DirectionalKey::from_robot_action(movement.1));
        let instruction = direction_precomp.get(&movement).unwrap();
        let action = (RobotAction::Activate, instruction[0]);

        if let None = new_moves_with_count.get(&action) {
            new_moves_with_count.insert(action, count);
        } else if let Some(action) = new_moves_with_count.get_mut(&action) {
            *action += count;
        }

        for i in 0..instruction.len() - 1 {
            let action = (instruction[i], instruction[i + 1]);

            if let None = new_moves_with_count.get(&action) {
                new_moves_with_count.insert(action, count);
            } else if let Some(action) = new_moves_with_count.get_mut(&action) {
                *action += count;
            }
        }
    }

    new_moves_with_count
}

fn solve_iterations(iterations: usize, codes: &Vec<&str>, numpad_precomp: &PrecomputeMap<NumpadKey>, dir_precomp: &PrecomputeMap<DirectionalKey>) -> usize {
    let mut result = 0;

    for code in codes {
        let number = code_to_number(code);
        let mut code = code.chars()
            .map(NumpadKey::from_char)
            .map(Option::unwrap)
            .collect::<Vec<_>>();
        code.insert(0, NumpadKey::Activate);

        let mut move_counts = solve_numpad_movements(&code, numpad_precomp);

        for _ in 0..iterations {
            move_counts = solve_direction_movements(move_counts, dir_precomp);
        }

        let total_movements = move_counts.values().fold(0, |sum, &count| sum + count);
        result += number * total_movements;
    }

    result
}

impl Solution for KeypadConundrum {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let mut num_pad = HashMap::new();
        {
            num_pad.insert(Position::new(0, 0), NumpadKey::Number7);
            num_pad.insert(Position::new(1, 0), NumpadKey::Number8);
            num_pad.insert(Position::new(2, 0), NumpadKey::Number9);
            num_pad.insert(Position::new(0, 1), NumpadKey::Number4);
            num_pad.insert(Position::new(1, 1), NumpadKey::Number5);
            num_pad.insert(Position::new(2, 1), NumpadKey::Number6);
            num_pad.insert(Position::new(0, 2), NumpadKey::Number1);
            num_pad.insert(Position::new(1, 2), NumpadKey::Number2);
            num_pad.insert(Position::new(2, 2), NumpadKey::Number3);
            num_pad.insert(Position::new(0, 3), NumpadKey::Blank);
            num_pad.insert(Position::new(1, 3), NumpadKey::Number0);
            num_pad.insert(Position::new(2, 3), NumpadKey::Activate);
        }

        let mut directional_pad = HashMap::new();
        {
            directional_pad.insert(Position::new(0, 0), DirectionalKey::Blank);
            directional_pad.insert(Position::new(1, 0), DirectionalKey::Up);
            directional_pad.insert(Position::new(2, 0), DirectionalKey::Activate);
            directional_pad.insert(Position::new(0, 1), DirectionalKey::Left);
            directional_pad.insert(Position::new(1, 1), DirectionalKey::Down);
            directional_pad.insert(Position::new(2, 1), DirectionalKey::Right);
        }

        let numpad_precomp = precompute_best_numpad_moves(&num_pad);
        let directional_precomp = precompute_best_numpad_moves(&directional_pad);

        let codes = input.lines().collect::<Vec<&str>>();

        let part1 = solve_iterations(2, &codes, &numpad_precomp, &directional_precomp);
        let part2 = solve_iterations(25, &codes, &numpad_precomp, &directional_precomp);

        solution!(part1, part2)
    }
}