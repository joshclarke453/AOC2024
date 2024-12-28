use std::fs;

fn process_input(char_array: Vec<char>) -> Vec<i32> {
    let mut disk = Vec::new();
    for i in (0..char_array.len()).step_by(2) {
        let file_id = (i / 2) as i32;
        let file_size = char_array[i]
            .to_digit(10)
            .expect(&format!("Failed to parse file size: {}", char_array[i]));
        let blank_size = char_array[i + 1]
            .to_digit(10)
            .expect(&format!("Failed to parse file size: {}", char_array[i + 1]));
        for _ in 0..file_size {
            disk.push(file_id);
        }
        for _ in 0..blank_size {
            disk.push(-1);
        }
    }
    return disk;
}

//[0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5, 5, 5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9]
fn defrag(processed_arr: Vec<i32>) -> Vec<i32> {
    let mut defragged = processed_arr.clone();

    let mut left_index = 0;
    let mut right_index = defragged.len() - 1;
    loop {
        while left_index < (defragged.len() - 1) {
            if left_index == right_index {
                return defragged;
            };
            match defragged[left_index] {
                -1 => {
                    break;
                }
                _ => {
                    left_index += 1;
                    continue;
                }
            }
        }

        while right_index > 0 {
            if left_index == right_index {
                return defragged;
            };
            match defragged[right_index] {
                -1 => {
                    right_index -= 1;
                    continue;
                }
                _ => {
                    break;
                }
            }
        }

        defragged[left_index] = defragged[right_index];
        defragged[right_index] = -1;
    }
}

fn defrag2(processed_arr: Vec<i32>) -> Vec<i32> {
    let mut defragged = processed_arr.clone();

    let mut left_index = 0;
    let mut right_index = defragged.len() - 1;
    loop {
        println!("LI: {}", left_index);
        println!("RI: {}", right_index);
        let mut left_num = 0;
        let mut right_num = 0;
        while left_index < (defragged.len() - 1) {
            if left_index == right_index {
                return defragged;
            };
            match defragged[left_index] {
                -1 => {
                    let mut i = left_index.clone();
                    while defragged[i] == -1 {
                        left_num += 1;
                        i += 1;
                    }
                    left_index += left_num;
                    break;
                }
                _ => {
                    left_index += 1;
                    continue;
                }
            }
        }

        while right_index > 0 {
            if left_index == right_index {
                return defragged;
            };
            match defragged[right_index] {
                -1 => {
                    right_index -= 1;
                    continue;
                }
                _ => {
                    let mut k = right_index.clone();
                    let ch = defragged[k];
                    while defragged[k] == ch {
                        right_num += 1;
                        k -= 1;
                    }
                    right_index -= right_num;
                    break;
                }
            }
        }

        if left_num > right_num {
            defragged[left_index] = defragged[right_index];
            defragged[right_index] = -1;
        }
        println!("{:?}", defragged);
        println!("LN: {}", left_num);
        println!("RN: {}", right_num);
    }
}

fn checksum(defragged_vec: Vec<i32>) -> i64 {
    let mut checksum = 0;
    for i in 0..defragged_vec.len() {
        checksum += i as i64 * defragged_vec[i] as i64;
    }
    return checksum;
}
fn main() {
    let input_content = "23331331214141314020"; //fs::read_to_string("./input.txt").expect("Should have been able to read the file.");
    let input_chars: Vec<char> = input_content.chars().collect();
    let processed = process_input(input_chars.clone());
    let defrag: Vec<i32> = defrag(processed.clone())
        .into_iter()
        .filter(|&x| x != -1)
        .collect();
    let checksum1 = checksum(defrag);
    let defrag2: Vec<i32> = defrag2(processed.clone())
        .into_iter()
        .filter(|&x| x != -1)
        .collect();
    let checksum2 = checksum(defrag2);
    println!("Checksum: {}", checksum1);
    println!("Checksum: {}", checksum2);
}
