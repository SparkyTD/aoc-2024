use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use crate::utils::matrix::Matrix;
use crate::utils::solution::{solution, Solution};

/// <b>Part 1:</b> Count the number of antinodes caused by the antennas
/// <br/><br/><b>Part 2:</b> Count the number of extended antinodes
#[derive(Default)]
pub struct ResonantCollinearity;

impl Solution for ResonantCollinearity {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let map = Matrix::<char>::from_text(&input);
        let mut antennas: HashMap<(u8, u8), char> = HashMap::new();
        let mut antinodes_simple: HashSet<(u8, u8)> = HashSet::new();
        let mut antinodes_all: HashSet<(u8, u8)> = HashSet::new();

        map.each(|x, y, ch| {
            if *ch != '.' {
                antennas.insert((*x as u8, *y as u8), *ch);
            }
        });

        for (&a1_pos, &a1_freq) in &antennas {
            for (&a2_pos, &a2_freq) in &antennas {
                if (a1_pos.0, a1_pos.1) == (a2_pos.0, a2_pos.1) || a1_freq != a2_freq {
                    continue;
                }

                antinodes_all.insert((a1_pos.0, a1_pos.1));
                antinodes_all.insert((a2_pos.0, a2_pos.1));

                let diff_x = a2_pos.0 as i16 - a1_pos.0 as i16;
                let diff_y = a2_pos.1 as i16 - a1_pos.1 as i16;

                let mut antinode_x = a2_pos.0 as i16 + diff_x;
                let mut antinode_y = a2_pos.1 as i16 + diff_y;

                let mut first_antinode = true;
                while !(antinode_x < 0 || antinode_y < 0 || antinode_x >= map.width() as i16 || antinode_y >= map.height() as i16) {
                    if first_antinode {
                        antinodes_simple.insert((antinode_x as u8, antinode_y as u8));
                    }

                    antinodes_all.insert((antinode_x as u8, antinode_y as u8));

                    antinode_x += diff_x;
                    antinode_y += diff_y;

                    first_antinode = false;
                }
            }
        }

        solution!(antinodes_simple.len(), antinodes_all.len())
    }
}