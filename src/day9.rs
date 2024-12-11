use std::fmt::{Debug, Write};

#[derive(Clone)]
enum Block {
    FreeSpace,
    File(u32),
}

enum Area {
    FreeSpace(u8),
    FileData(u8, u32), // length, data
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Block::FreeSpace => '.',
            Block::File(n) => std::char::from_digit(*n, 10).unwrap(),
        })
    }
}

impl Debug for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let length = match self {
            Area::FreeSpace(l) => *l,
            Area::FileData(l, _) => *l,
        };

        for _ in 0..length {
            f.write_char(match self {
                Area::FreeSpace(_) => '.',
                Area::FileData(_, id) => std::char::from_digit(*id, 10).unwrap(),
            })?;
        }

        Ok(())
    }
}

pub fn day_9() {
    let input = include_str!("../data/day9.txt")
        .chars().filter(|c| c.is_digit(10) || *c == '.')
        .map(|c| c.to_digit(10).unwrap() as u8);

    let mut compressed = vec![];

    // Decompress the data
    let mut last_id: u32 = 0;
    let mut extracted = vec![];
    for b in input.clone() {
        let is_file = last_id % 2 == 0;
        for _ in 0..b {
            extracted.push(match is_file {
                true => Block::File(last_id / 2),
                false => Block::FreeSpace,
            });
        }
        compressed.push(match is_file {
            true => Area::FileData(b, last_id / 2),
            false => Area::FreeSpace(b),
        });
        last_id += 1;
    }

    // Rearrange the data blocks
    let mut start: usize = 0;
    let mut end = extracted.len() - 1;
    let mut extracted_2 = extracted.iter().clone().cloned().collect::<Vec<Block>>();
    while start <= end {
        if let (Block::FreeSpace, Block::File(f)) = (&extracted_2[start], &extracted_2[end]) {
            (extracted_2[start], extracted_2[end]) = (Block::File(*f), Block::FreeSpace);
            end -= 1;
        }

        if let Block::File(_) = extracted_2[start] {
            start += 1;
        }

        if let Block::FreeSpace = extracted_2[end] {
            end -= 1;
        }
    }

    // Calculate the checksum
    let mut total: u64 = 0;
    let mut position = 0;
    for b in extracted_2 {
        if let Block::File(f) = b {
            total += f as u64 * position;
        }
        position += 1;
    }
    println!("Checksum: {}", total);

    // Part 2
    for end in (0..compressed.len()).rev() {
        if let Area::FileData(length, data) = &compressed[end] {
            let length = *length;
            let data = *data;
            for start in 0..end {
                if let Area::FreeSpace(space) = &compressed[start] {
                    let space = *space;
                    if length == space {
                        compressed.swap(start, end);
                        break;
                    } else if length < space {
                        compressed[start] = Area::FileData(length, data);
                        compressed.insert(start + 1, Area::FreeSpace(space - length));
                        compressed[end + 1] = Area::FreeSpace(length);
                        break;
                    }
                }
            }
        }
    }

    // Calculate the checksum
    let mut compressed = compressed.iter().flat_map(|a| match a {
        Area::FreeSpace(l) => vec![Block::FreeSpace; *l as usize],
        Area::FileData(l, data) => vec![Block::File(*data); *l as usize],
    }).collect::<Vec<_>>();
    /*println!("{:?}", compressed.iter().map(|b| match b {
        Block::FreeSpace => '.',
        Block::File(n) => std::char::from_digit(*n, 10).unwrap(),
    }).collect::<String>());*/

    let mut total: u64 = 0;
    let mut position = 0;
    for b in compressed {
        if let Block::File(f) = b {
            total += f as u64 * position;
        }
        position += 1;
    }
    println!("Checksum: {}", total);
}