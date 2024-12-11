use std::fs;

fn test_index_error(i: i32, j: i32) -> bool {
    let width = 140;
    let height = 140;
    if i < 0 || i >= width {
        return false;
    }
    if j < 0 || j >= height {
        return false;
    }
    return true;
}

fn find_all_words(lines: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let int_i = i as i32;
            let int_j = j as i32;
            if test_index_error(int_i, int_j) && lines[i][j] != 'X' {
                continue;
            }

            if test_index_error(int_i - 1, int_j - 1) && lines[i - 1][j - 1] == 'M' {
                if test_index_error(int_i - 2, int_j - 2) && lines[i - 2][j - 2] == 'A' {
                    if test_index_error(int_i - 3, int_j - 3) && lines[i - 3][j - 3] == 'S' {
                        count += 1;
                    }
                }
            }

            if test_index_error(int_i, int_j - 1) && lines[i][j - 1] == 'M' {
                if test_index_error(int_i, int_j - 2) && lines[i][j - 2] == 'A' {
                    if test_index_error(int_i, int_j - 3) && lines[i][j - 3] == 'S' {
                        count += 1;
                    }
                }
            }

            if test_index_error(int_i + 1, int_j - 1) && lines[i + 1][j - 1] == 'M' {
                if test_index_error(int_i + 2, int_j - 2) && lines[i + 2][j - 2] == 'A' {
                    if test_index_error(int_i + 3, int_j - 3) && lines[i + 3][j - 3] == 'S' {
                        count += 1;
                    }
                }
            }

            if test_index_error(int_i - 1, int_j) && lines[i - 1][j] == 'M' {
                if test_index_error(int_i - 2, int_j) && lines[i - 2][j] == 'A' {
                    if test_index_error(int_i - 3, int_j) && lines[i - 3][j] == 'S' {
                        count += 1;
                    }
                }
            }

            if test_index_error(int_i + 1, int_j) && lines[i + 1][j] == 'M' {
                if test_index_error(int_i + 2, int_j) && lines[i + 2][j] == 'A' {
                    if test_index_error(int_i + 3, int_j) && lines[i + 3][j] == 'S' {
                        count += 1;
                    }
                }
            }

            if test_index_error(int_i - 1, int_j + 1) && lines[i - 1][j + 1] == 'M' {
                if test_index_error(int_i - 2, int_j + 2) && lines[i - 2][j + 2] == 'A' {
                    if test_index_error(int_i - 3, int_j + 3) && lines[i - 3][j + 3] == 'S' {
                        count += 1;
                    }
                }
            }

            if test_index_error(int_i, int_j + 1) && lines[i][j + 1] == 'M' {
                if test_index_error(int_i, int_j + 2) && lines[i][j + 2] == 'A' {
                    if test_index_error(int_i, int_j + 3) && lines[i][j + 3] == 'S' {
                        count += 1;
                    }
                }
            }

            if test_index_error(int_i + 1, int_j + 1) && lines[i + 1][j + 1] == 'M' {
                if test_index_error(int_i + 2, int_j + 2) && lines[i + 2][j + 2] == 'A' {
                    if test_index_error(int_i + 3, int_j + 3) && lines[i + 3][j + 3] == 'S' {
                        count += 1;
                    }
                }
            }
        }
    }
    return count;
}

fn find_all_x_mas(lines: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let int_i = i as i32;
            let int_j = j as i32;

            if !test_index_error(int_i - 1, int_j - 1) || !test_index_error(int_i + 1, int_j + 1) {
                continue;
            }

            if lines[i][j] != 'A' {
                continue;
            }

            if lines[i - 1][j - 1] == 'M'
                && lines[i - 1][j + 1] == 'M'
                && lines[i + 1][j - 1] == 'S'
                && lines[i + 1][j + 1] == 'S'
            {
                count += 1;
                continue;
            }

            if lines[i - 1][j - 1] == 'S'
                && lines[i - 1][j + 1] == 'M'
                && lines[i + 1][j - 1] == 'S'
                && lines[i + 1][j + 1] == 'M'
            {
                count += 1;
                continue;
            }

            if lines[i - 1][j - 1] == 'S'
                && lines[i - 1][j + 1] == 'S'
                && lines[i + 1][j - 1] == 'M'
                && lines[i + 1][j + 1] == 'M'
            {
                count += 1;
                continue;
            }

            if lines[i - 1][j - 1] == 'M'
                && lines[i - 1][j + 1] == 'S'
                && lines[i + 1][j - 1] == 'M'
                && lines[i + 1][j + 1] == 'S'
            {
                count += 1;
                continue;
            }
        }
    }
    return count;
}

fn main() {
    let input_content =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file.");
    let strs = input_content.lines().collect::<Vec<&str>>();
    let chars = strs
        .iter()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let count = find_all_words(chars.clone());
    println!("{}", count);
    let count_x = find_all_x_mas(chars.clone());
    println!("{}", count_x);
}
