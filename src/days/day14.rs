use std::fmt::Display;
use crate::utils::matrix::Matrix;
use crate::utils::solution::{solution, Solution};

#[derive(Debug)]
struct Robot {
    init_pos_x: usize,
    init_pos_y: usize,

    pos_x: usize,
    pos_y: usize,

    velocity_x: i32,
    velocity_y: i32,
}

fn modulo(x: i32, m: i32) -> i32 {
    (x % m + m) % m
}

/// <b>Part 1:</b> Calculate the safety factor of robots moving on a 2D grid after 100 seconds
/// <br/><br/><b>Part 2:</b> Count the fewest number of seconds before robots display an Easter egg
#[derive(Default)]
pub struct RestroomRedoubt;

impl Solution for RestroomRedoubt {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let mut robots = Vec::new();
        let mut space_width = 0;
        let mut space_height = 0;
        for line in input.lines() {
            let parts = line.split(['=', ' ', ',']).collect::<Vec<_>>();
            let mut robot = Robot {
                init_pos_x: parts[1].parse().unwrap(),
                init_pos_y: parts[2].parse().unwrap(),
                velocity_x: parts[4].parse().unwrap(),
                velocity_y: parts[5].parse().unwrap(),
                pos_x: 0,
                pos_y: 0,
            };

            robot.pos_x = robot.init_pos_x;
            robot.pos_y = robot.init_pos_y;

            space_width = robot.pos_x.max(space_width);
            space_height = robot.pos_y.max(space_height);
            robots.push(robot);
        }

        space_width += 1;
        space_height += 1;

        let simulation_count = 100;

        // Part 1
        for robot in &mut robots {
            robot.pos_x = modulo(robot.pos_x as i32 + robot.velocity_x * simulation_count, space_width as i32) as usize;
            robot.pos_y = modulo(robot.pos_y as i32 + robot.velocity_y * simulation_count, space_height as i32) as usize;
        }

        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;

        for robot in &robots {
            if robot.pos_x < space_width / 2 && robot.pos_y < space_height / 2 {
                q1 += 1;
            }
            if robot.pos_x > space_width / 2 && robot.pos_y < space_height / 2 {
                q2 += 1;
            }
            if robot.pos_x < space_width / 2 && robot.pos_y > space_height / 2 {
                q3 += 1;
            }
            if robot.pos_x > space_width / 2 && robot.pos_y > space_height / 2 {
                q4 += 1;
            }
        }

        // Part 2
        let mut matrix = Matrix::<u32>::new(space_width, space_height);
        for robot in &mut robots {
            robot.pos_x = robot.init_pos_x;
            robot.pos_y = robot.init_pos_y;
            *matrix.get_mut(robot.pos_x, robot.pos_y).unwrap() += 1;
        }

        let candidate_threshold = space_width * space_height / 520;
        let sanity_counter = 20000;
        let mut count = 0;
        loop {
            // Simulate one second
            let mut candidates = 0;
            for robot in &mut robots {
                *matrix.get_mut(robot.pos_x, robot.pos_y).unwrap() -= 1;
                robot.pos_x = modulo(robot.pos_x as i32 + robot.velocity_x, space_width as i32) as usize;
                robot.pos_y = modulo(robot.pos_y as i32 + robot.velocity_y, space_height as i32) as usize;
                *matrix.get_mut(robot.pos_x, robot.pos_y).unwrap() += 1;

                if robot.pos_x > 0 && robot.pos_y > 0 && robot.pos_x < space_width - 1 && robot.pos_y < space_height - 1 {
                    if *matrix.get_mut(robot.pos_x - 1, robot.pos_y).unwrap() > 0 &&
                        *matrix.get_mut(robot.pos_x, robot.pos_y - 1).unwrap() > 0 &&
                        *matrix.get_mut(robot.pos_x + 1, robot.pos_y).unwrap() > 0 &&
                        *matrix.get_mut(robot.pos_x, robot.pos_y + 1).unwrap() > 0 {
                        candidates += 1;
                    }
                }
            }

            if candidates >= candidate_threshold || count == sanity_counter {
                break;
            }

            count += 1;
        }

        solution!(q1 * q2 * q3 * q4, count + 1)
    }
}