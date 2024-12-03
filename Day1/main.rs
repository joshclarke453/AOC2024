use std::fs;

fn main() {
    let input_content = fs::read_to_string("./input.txt").expect("Should have been able to read the file.");
    let lines = input_content.lines();
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    for a in lines {
        let split_ins: Vec<&str> = a.split("   ").collect();
        list1.push(split_ins[0].parse().unwrap());
        list2.push(split_ins[1].parse().unwrap());
    }
    list1.sort();
    list2.sort();
    let mut sum = 0;
    let mut sim = 0;
    for n in 0..list1.len() {
        sum += (list1[n] - list2[n]).abs();
        sim += list1[n] * (list2.clone().into_iter().filter(|&x| x == list1[n]).collect::<Vec<i32>>().len() as i32);
    }
    println!("Total Distance: {sum}");
    println!("Total Similarity: {sim}");
}