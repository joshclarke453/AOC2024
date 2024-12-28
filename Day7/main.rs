use std::fs;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use itertools::Itertools;

fn run(characters: Vec<&str>) {
    let input_content = fs::read_to_string("./input.txt").expect("Should have been able to read the file.");
    let lines = input_content.lines();
    let mut total = 0;
    for a in lines {
        let split_ins: Vec<&str> = a.split(":").collect();
        let res: i64 = split_ins[0].parse().expect("1");
        let ins: Vec<&str> = split_ins[1].trim().split(" ").collect();

        let n= ins.len()-1; // The number of combinations
        
        let mut combinations : Vec<_> = (2..n).fold(
            characters.iter().cartesian_product(characters.iter()).map(|(&a, &b)| a.to_owned() + b).collect(),
            |acc, _| acc.into_iter().cartesian_product(characters.iter()).map(|(a, b)| a.to_owned() + b).collect()
        );

        match n {
            1 => { combinations = vec!["*".to_string(), "+".to_string(), "|".to_string()]; }
            _ => {}
        }

        let matches = Arc::new(Mutex::new(0));

        combinations.par_iter().for_each(|x| {
            let mut temp_res: i64 = ins[0].parse().expect("Failed to parse i64 result from input");
            let temp: Vec<char> = x.chars().collect();
            for i in 0..n {
                match temp[i] {
                    '+' => {temp_res = temp_res + (ins[i+1]).parse::<i64>().expect("Couldn't parse int64 +")},
                    '*' => {temp_res = temp_res * (ins[i+1]).parse::<i64>().expect("Couldn't parse int64 *")}
                    '|' => {temp_res = (temp_res.to_string() + ins[i+1]).parse::<i64>().expect("Couldn't parse int64 |")}
                    _ => {println!("Something fucked up")}
                }
            }
            
            if temp_res == res {
                let mut matches_lock = matches.lock().expect("Couldn't lock matches to modify");
                *matches_lock += temp_res;
            }
        });

        if *matches.lock().expect("Couldn't lock matches to access") > 0 {
            total += res;
        }
    }
    println!("{:?}", total)
}
fn main() {    
    run(vec!["*", "+"]);
    run(vec!["*", "+", "|"]);
}