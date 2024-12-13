use std::{fs, vec};

fn change_guard_pos(global_guard_pos: &mut Vec<usize>, new_pos: Vec<usize>) {
    *global_guard_pos = new_pos;
}

fn turn_guard_right(chars: &mut Vec<Vec<char>>, pos: Vec<usize>) {
    let i = pos[0];
    let j = pos[1];
    if chars[i][j] == '^' {
        replace_tile(chars, vec![i, j], '>');
    } else if chars[i][j] == '>' {
        replace_tile(chars, vec![i, j], 'v');
    } else if chars[i][j] == 'v' {
        replace_tile(chars, vec![i, j], '<');
    } else if chars[i][j] == '<' {
        replace_tile(chars, vec![i, j], '^');
    }
}

fn find_guard(chars: &Vec<Vec<char>>) -> Vec<usize> {
    let mut pos = Vec::new();
    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            match "v^<>".find(chars[i][j]) {
                Some(_) => pos = vec![i, j],
                None => {}
            }
        }
    }
    return pos;
}

fn replace_tile(chars: &mut Vec<Vec<char>>, pos: Vec<usize>, replace_char: char) {
    let _ = std::mem::replace(&mut chars[pos[0]][pos[1]], replace_char);
}

fn progress(chars: &mut Vec<Vec<char>>, global_guard_pos: &mut Vec<usize>) -> bool {
    let i = global_guard_pos[0];
    let j = global_guard_pos[1];
    if chars[i][j] == '^' {
        if i == 0 {
            replace_tile(chars, vec![i, j], 'X');
            return false;
        } else {
            if chars[i-1][j] == '#' {
                turn_guard_right(chars, global_guard_pos.clone());
            } else {
                change_guard_pos(global_guard_pos, vec![i-1, j]);
                replace_tile(chars, global_guard_pos.clone(), '^');
                replace_tile(chars, vec![i, j], 'X');
            }
        }
    } else if chars[i][j] == '>' {
        if j == chars[i].len()-1 {
            replace_tile(chars, vec![i, j], 'X');
            return false;
        } else {
            if chars[i][j+1] == '#' {
                turn_guard_right(chars, global_guard_pos.clone());
            } else {
                change_guard_pos(global_guard_pos, vec![i, j+1]);
                replace_tile(chars, global_guard_pos.clone(), '>');
                replace_tile(chars, vec![i, j], 'X');
            }
        }
    } else if chars[i][j] == 'v' {
        if i == chars.len()-1 {
            replace_tile(chars, vec![i, j], 'X');
            return false;
        } else {
            if chars[i+1][j] == '#' {
                turn_guard_right(chars, global_guard_pos.clone());
            } else {
                change_guard_pos(global_guard_pos, vec![i+1, j]);
                replace_tile(chars, global_guard_pos.clone(), 'v');
                replace_tile(chars, vec![i, j], 'X');
            }
        }
    } else if chars[i][j] == '<' {
        if j == 0 {
            replace_tile(chars, vec![i, j], 'X');
            return false;
        } else {
            if chars[i][j-1] == '#' {
                turn_guard_right(chars, global_guard_pos.clone());
            } else {
                change_guard_pos(global_guard_pos, vec![i, j-1]);
                replace_tile(chars, global_guard_pos.clone(), '<');
                replace_tile(chars, vec![i, j], 'X');
            }
        }
    }
    return true;
}

fn count_distinct(chars: &mut Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for x in chars.clone() {
        count += x.into_iter().filter(|c| *c == 'X').count();
    }
    return count as i32;
}

fn main() {
    let mut global_guard_pos = Vec::new();
    let input_content =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file.");
    let strs = input_content.lines().collect::<Vec<&str>>();
    let mut chars = strs
        .iter()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();
    change_guard_pos(&mut global_guard_pos, find_guard(&chars));
    let mut guard_left = false;
    while !guard_left {
        if !progress(&mut chars, &mut global_guard_pos) {
            guard_left = true;
        }
    }
    
    println!("{}", count_distinct(&mut chars));
}
