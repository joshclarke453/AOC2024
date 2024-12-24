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
                'a'..'z' | 'A'..'Z' | '0'..'9' => { key_value.insert(chars[x][y], (x, y)); }
                _ => {}
            }
        }
    }
    return key_value;
}

fn main() {
    let input_content =
        fs::read_to_string("./in.txt").expect("Should have been able to read the file.");
    let strs = input_content.lines().collect::<Vec<&str>>();
    //chars: [y,x] where y is the vertical, x is the horizontal
    let mut chars = strs
        .iter()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let temp = get_keys_and_pos(&mut chars);
    println!("{:?}", temp)
}
