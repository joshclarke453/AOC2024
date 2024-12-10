use std::{fs, str::Lines};

fn test_seq(seq: Vec<i32>) -> bool {
    let mut asc_sort = seq.clone();
    asc_sort.sort();
    let mut desc_sort = seq.clone();
    desc_sort.sort_by(|a, b| b.cmp(a));
    if !(seq.clone().into_iter().eq(asc_sort.clone().into_iter()) || seq.clone().into_iter().eq(desc_sort.clone().into_iter())) {
        return false;
    }
    let mut good = true;
    for x in 1..seq.len() {
        if 
            (0 == (seq[x] - seq[x-1]).abs()) || 
            ((seq[x] - seq[x-1]).abs() >= 4)
        {
            good = false;
            break;
        }
    }
    if !good {
        return false;
    }
    return true;
}

fn pt1(lines: Lines) {
    let mut sum = 0;
    for a in lines {
        let orig: Vec<i32> = a.split_whitespace().map(|x|->i32{x.parse().unwrap()}).collect();
        let good = test_seq(orig);
        if good {
            sum += 1;
        }
    }
    println!("{sum}");
}

fn pt2(lines: Lines) {
    let mut sum = 0;
    for a in lines {
        let orig: Vec<i32> = a.split_whitespace().map(|x|->i32{x.parse().unwrap()}).collect();
        let good = test_seq(orig.clone());
        if good {
            sum += 1;
            continue;
        }
        for (index, _value) in orig.clone().iter().enumerate() {
            let mut replaced = orig.clone();
            replaced.splice(index..index+1, []);
            let rep_good = test_seq(replaced);
            if rep_good {
                sum += 1;
                break;
            }
        }
    }
    println!("{sum}");
}

fn main() {
    let input_content = fs::read_to_string("./input.txt").expect("Should have been able to read the file.");
    let lines = input_content.lines();
    pt1(lines.clone());
    pt2(lines.clone());
}
