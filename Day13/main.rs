use std::{fs, sync::{Arc, Mutex}};
use rayon::prelude::*;

#[derive(Clone)]
#[derive(Debug)]
struct SystemObject {
    prize: (i32, i32),
    a: (i32, i32),
    b: (i32, i32)
}

fn make_system_objs(lines: Vec<&str>) -> Vec<SystemObject> {
    let mut count = 0;
    let mut system_objs = Vec::new();
    while count < lines.len() {
        let a_split: Vec<&str> = lines[count][10..].split(", ").collect();
        let a_button: (i32, i32) = (a_split[0][2..].parse().unwrap(), a_split[1][2..].parse().unwrap());

        let b_split: Vec<&str> = lines[count+1][10..].split(", ").collect();
        let b_button: (i32, i32) = (b_split[0][2..].parse().unwrap(), b_split[1][2..].parse().unwrap());

        let prize_split: Vec<&str> = lines[count+2][7..].split(", ").collect();
        let prize_tuple: (i32, i32) = (prize_split[0][2..].parse().unwrap(), prize_split[1][2..].parse().unwrap());

        let system_obj = SystemObject {
            a: a_button,
            b: b_button,
            prize: prize_tuple
        };

        system_objs.push(system_obj);
        count += 4;
    }
    return system_objs;
}

fn solve_system(system_obj: SystemObject) -> (SystemObject, (i32, i32), bool) {
    for a in 0..=100 {
        for b in 0..=100 {
            if  (system_obj.a.0 * a + system_obj.b.0 * b == system_obj.prize.0) && 
                (system_obj.a.1 * a + system_obj.b.1 * b == system_obj.prize.1)
            {
                return (system_obj, (a, b), true);
            }
        }
    }
    return (system_obj, (0,0), false);
}

fn calculate(system_objs: Vec<SystemObject>) {
    let possible = Arc::new(Mutex::new(0));
    let cost = Arc::new(Mutex::new(0));
    system_objs.par_iter().for_each(|x| {
        let result = solve_system(x.clone());
        match result.2 {
            true => {
                let mut possible_lock = possible.lock().expect("Couldn't lock matches to modify");
                *possible_lock += 1;


                let mut cost_lock = cost.lock().expect("Couldn't lock matches to modify");
                *cost_lock += result.1.0 * 3 + result.1.1 * 1;
            },
            false => {}
        }
    });
    
    println!("{:?}", possible);
    println!("{:?}", cost);
}

fn pt2() {

}

fn main() {
    let input_content =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file.");
    let strs = input_content.lines().collect::<Vec<&str>>();
    let system_objs = make_system_objs(strs);
    println!("{:?}", system_objs);

    calculate(system_objs.clone());
}