use std::collections::HashSet;

fn get_destination_count(map: &Vec<Vec<u8>>, start_x: i16, start_y: i16, prev_value: Option<u8>) -> Option<HashSet<(i16, i16)>> {
    if start_x < 0 || start_x >= map[0].len() as i16 || start_y < 0 || start_y >= map.len() as i16 {
        return None;
    }

    let current_value = map[start_y as usize][start_x as usize];
    if let Some(prev_value) = prev_value {
        if current_value != prev_value + 1 {
            return None;
        }
    }

    if current_value == 9 {
        let mut hash_set = HashSet::new();
        hash_set.insert((start_x, start_y));
        return Some(hash_set);
    }

    let mut hash_set = HashSet::new();
    let up = get_destination_count(map, start_x, start_y - 1, Some(current_value));
    let down = get_destination_count(map, start_x, start_y + 1, Some(current_value));
    let left = get_destination_count(map, start_x - 1, start_y, Some(current_value));
    let right = get_destination_count(map, start_x + 1, start_y, Some(current_value));

    if let Some(up) = up {
        hash_set.extend(up);
    }
    if let Some(down) = down {
        hash_set.extend(down);
    }
    if let Some(left) = left {
        hash_set.extend(left);
    }
    if let Some(right) = right {
        hash_set.extend(right);
    }

    Some(hash_set)
}

fn get_unique_trail_count(map: &Vec<Vec<u8>>, start_x: i16, start_y: i16, prev_value: Option<u8>) -> u8 {
    if start_x < 0 || start_x >= map[0].len() as i16 || start_y < 0 || start_y >= map.len() as i16 {
        return 0;
    }

    let current_value = map[start_y as usize][start_x as usize];
    if let Some(prev_value) = prev_value {
        if current_value != prev_value + 1 {
            return 0;
        }
    }

    if current_value == 9 {
        return 1;
    }

    let up = get_unique_trail_count(map, start_x, start_y - 1, Some(current_value));
    let down = get_unique_trail_count(map, start_x, start_y + 1, Some(current_value));
    let left = get_unique_trail_count(map, start_x - 1, start_y, Some(current_value));
    let right = get_unique_trail_count(map, start_x + 1, start_y, Some(current_value));

    up + down + left + right
}

pub fn day_10() {
    let input = include_str!("../data/day10.txt");

    let mut map: Vec<Vec<u8>> = Vec::new(); // [y][x]
    let mut trailheads: Vec<(u8, u8)> = Vec::new(); // (x, y)

    for line in input.lines() {
        let mut line_vec = Vec::new();
        for (i, ch) in line.char_indices() {
            let height = if let Some(height) = ch.to_digit(10) {
                height
            } else {
                255
            } as u8;
            line_vec.push(height);

            if height == 0 {
                trailheads.push((i as u8, map.len() as u8));
            }
        }
        map.push(line_vec);
    }

    let score = trailheads.iter()
        .map(|&t| get_destination_count(&map, t.0 as i16, t.1 as i16, None).map_or(0, |h| h.len()))
        .sum::<usize>();
    println!("Score: {}", score);

    let rating = trailheads.iter()
        .map(|&t| get_unique_trail_count(&map, t.0 as i16, t.1 as i16, None) as usize)
        .sum::<usize>();
    println!("Rating: {}", rating);
}