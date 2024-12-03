use std::fs;

fn main() {
    let input_content = fs::read_to_string("./input.txt").expect("Should have been able to read the file.");
    let lines = input_content.lines();
    let mut sum = 0;
    for a in lines {
        let orig: Vec<i32> = a.split_whitespace().map(|x|->i32{x.parse().unwrap()}).collect();
        let mut asc_sort = orig.clone();
        asc_sort.sort();
        let mut desc_sort = orig.clone();
        desc_sort.sort_by(|a, b| b.cmp(a));
        if !(orig.clone().into_iter().eq(asc_sort.clone().into_iter()) || orig.clone().into_iter().eq(desc_sort.clone().into_iter())) {
            continue;
        }
        let mut good = true;
        for x in 1..orig.len() {
            if 
                (0 == (orig[x] - orig[x-1]).abs()) || 
                ((orig[x] - orig[x-1]).abs() >= 4)
            {
                good = false;
                break;
            }
        }
        if !good {
            continue;
        }
        sum += 1;
    }
    println!("{sum}");
}