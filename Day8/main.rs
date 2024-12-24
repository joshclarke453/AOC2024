use itertools::{iproduct, Itertools};
use multimap::MultiMap;

use std::fs;

/**
 * Returns a MultiMap ex.
 * {
 *      'a': [(7,3), (1,1)]
 *      'b': [(4,5)]
 *      'c': [(1,3), (2,2)]
 * }
 */
fn get_keys_and_pos(chars: &mut Vec<Vec<char>>) -> MultiMap<char, (usize, usize)> {
    let mut key_value = MultiMap::new();
    for y in 0..chars.len() {
        for x in 0..chars[y].len() {
            match chars[x][y] {
                '.' => {}
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    key_value.insert(chars[x][y], (x, y));
                }
                _ => {}
            }
        }
    }
    return key_value;
}

fn p1_find_antinodes(
    keymap: MultiMap<char, (usize, usize)>,
    bounds: (usize, usize),
) -> Vec<(i32, i32)> {
    let mut unique_antinodes = Vec::new();
    for key in keymap.keys() {
        let key_vals = keymap.get_vec(key).expect("Get keymap vec for {key}");
        match key_vals.len() {
            2 => continue,
            _ => {}
        }
        let cross: Vec<((usize, usize), (usize, usize))> = iproduct!(key_vals, key_vals)
            .map(|(a, b)| (a.clone(), b.clone()))
            .filter(|&x| x.0 != x.1)
            .collect();
        for k in cross {
            let temp_antinode1 = (
                k.0 .0 as i32 - (k.1 .0 as i32 - k.0 .0 as i32),
                k.0 .1 as i32 - (k.1 .1 as i32 - k.0 .1 as i32),
            );
            if temp_antinode1.1 >= 0
                && temp_antinode1.1 <= bounds.1 as i32
                && temp_antinode1.0 >= 0
                && temp_antinode1.0 <= bounds.0 as i32
            {
                unique_antinodes.push(temp_antinode1);
            }
            let temp_antinode2 = (
                k.1 .0 as i32 + (k.1 .0 as i32 - k.0 .0 as i32),
                k.1 .1 as i32 + (k.1 .1 as i32 - k.0 .1 as i32),
            );
            if temp_antinode2.1 >= 0
                && temp_antinode2.1 <= bounds.1 as i32
                && temp_antinode2.0 >= 0
                && temp_antinode2.0 <= bounds.0 as i32
            {
                unique_antinodes.push(temp_antinode2);
            }
        }
    }
    return unique_antinodes;
}

fn p2_find_antinodes(
    keymap: MultiMap<char, (usize, usize)>,
    bounds: (usize, usize),
) -> Vec<(i32, i32)> {
    let mut unique_antinodes = Vec::new();
    for key in keymap.keys() {
        let key_vals = keymap.get_vec(key).expect("Get keymap vec for {key}");
        match key_vals.len() {
            2 => continue,
            _ => {}
        }
        let cross: Vec<((usize, usize), (usize, usize))> = iproduct!(key_vals, key_vals)
            .map(|(a, b)| (a.clone(), b.clone()))
            .filter(|&x| x.0 != x.1)
            .collect();
        for k in cross {
            let mut t1_counter = 1;
            let mut t1_failed = false;
            unique_antinodes.push((k.0 .0 as i32, k.0 .1 as i32));
            unique_antinodes.push((k.1 .0 as i32, k.1 .1 as i32));
            while !t1_failed {
                let temp_antinode1 = (
                    k.0 .0 as i32 - t1_counter * (k.1 .0 as i32 - k.0 .0 as i32),
                    k.0 .1 as i32 - t1_counter * (k.1 .1 as i32 - k.0 .1 as i32),
                );
                if temp_antinode1.1 >= 0
                    && temp_antinode1.1 <= bounds.1 as i32
                    && temp_antinode1.0 >= 0
                    && temp_antinode1.0 <= bounds.0 as i32
                {
                    unique_antinodes.push(temp_antinode1);
                    t1_counter += 1;
                } else {
                    t1_failed = true;
                }
            }
            let mut t2_counter = 1;
            let mut t2_failed = false;
            while !t2_failed {
                let temp_antinode2 = (
                    k.1 .0 as i32 + t2_counter * (k.1 .0 as i32 - k.0 .0 as i32),
                    k.1 .1 as i32 + t2_counter * (k.1 .1 as i32 - k.0 .1 as i32),
                );
                if temp_antinode2.1 >= 0
                    && temp_antinode2.1 <= bounds.1 as i32
                    && temp_antinode2.0 >= 0
                    && temp_antinode2.0 <= bounds.0 as i32
                {
                    unique_antinodes.push(temp_antinode2);
                    t2_counter += 1;
                } else {
                    t2_failed = true;
                }
            }
        }
    }
    return unique_antinodes;
}

fn main() {
    let input_content =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file.");
    let strs = input_content.lines().collect::<Vec<&str>>();
    //chars: [y,x] where y is the vertical, x is the horizontal
    let mut chars = strs
        .iter()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let max_size = (chars.len() - 1, chars[0].len() - 1);
    let temp = get_keys_and_pos(&mut chars);
    let temp2: Vec<(i32, i32)> = p1_find_antinodes(temp.clone(), max_size)
        .into_iter()
        .unique()
        .collect();
    let temp3: Vec<(i32, i32)> = p2_find_antinodes(temp.clone(), max_size)
        .into_iter()
        .unique()
        .collect();
    println!("{}", temp2.len());
    println!("{}", temp3.len());
}
