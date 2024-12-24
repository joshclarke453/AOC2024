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

fn find_antinodes(
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
    let mut temp2: Vec<(i32, i32)> = find_antinodes(temp, max_size)
        .into_iter()
        .unique()
        .collect();
    temp2.sort();
    println!("{}", temp2.len());
}
