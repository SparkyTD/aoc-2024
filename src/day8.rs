use std::collections::{HashMap, HashSet};

pub fn day_8() {
    let input = include_str!("../data/day8.txt");

    let mut antennas: HashMap<(u8, u8), char> = HashMap::new();
    let mut antinodes_simple: HashSet<(u8, u8)> = HashSet::new();
    let mut antinodes_all: HashSet<(u8, u8)> = HashSet::new();

    let mut height = 0;
    let mut width = 0;
    for line in input.lines() {
        for (x, ch) in line.char_indices() {
            width = width.max(x + 1);
            if ch == '.' {
                continue;
            }

            antennas.insert((x as u8, height), ch);
        }
        height += 1;
    }

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
            while !(antinode_x < 0 || antinode_y < 0 || antinode_x >= width as i16 || antinode_y >= height as i16) {
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

    println!("P1 Antinode Count: {}", antinodes_simple.len());
    println!("P2 Antinode Count: {}", antinodes_all.len());
}