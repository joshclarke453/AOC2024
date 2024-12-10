use std::fs;

use regex::Regex;

fn pt1(input: String) {
    let re = Regex::new(r"mul\([\d]+,[\d]+\)").unwrap();
    let matches: Vec<&str> = re.find_iter(input.as_str()).map(|m| m.as_str()).collect();
    let mut sum = 0;
    for mut m in matches {
        m = &m[4..(m.len()-1)];  
        let nums_string: Vec<&str> = m.split(',').collect();
        let num1: i32 = nums_string[0].parse().unwrap();
        let num2: i32 = nums_string[1].parse().unwrap();
        sum += num1 * num2;
    }
    println!("{sum}")
}

fn pt2(input: String) {
    let re = Regex::new(r"don't\(\)([^.]*?)do\(\)").unwrap();
    let matches = re.replace_all(&input, "");
    pt1(matches.to_string());
}

fn main() {
    let input_content = fs::read_to_string("./input.txt").expect("Should have been able to read the file.");
    pt1(input_content.clone());
    pt2(input_content.clone());
}