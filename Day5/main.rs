use std::fs;

fn main() {
    let mut rules_vec: Vec<Vec<i32>> = Vec::new();
    let rules = fs::read_to_string("./rules.txt").expect("Should have been able to read the file.");
    let rules_lines = rules.lines();
    for r in rules_lines {
        let mut temp_rules: Vec<i32> = Vec::new();
        let split_rules: Vec<&str> = r.split("|").collect();
        temp_rules.push(split_rules[0].parse().unwrap());
        temp_rules.push(split_rules[1].parse().unwrap());
        rules_vec.push(temp_rules);
    }

    let input_content =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file.");
    let input_lines = input_content.lines();
    let mut good_middle_sum = 0;
    let mut bad_middle_sum = 0;
    for line in input_lines {
        let split_in: Vec<&str> = line.split(",").collect();
        let mut vec_in: Vec<i32> = Vec::new();
        for x in split_in {
            vec_in.push(x.parse().unwrap());
        }

        let mut bad = false;
        for j in 0..vec_in.len() {
            for k in 0..vec_in.len() {
                if j == k {
                    continue;
                }

                let mut cloned = vec_in.clone();
                for r in &rules_vec {
                    if cloned[j] == r[0] && cloned[k] == r[1] {
                        if j > k {
                            bad = true;
                            cloned.swap(j, k);
                        };
                    } else if cloned[j] == r[1] && cloned[j] == r[0] {
                        println!("{:?}", r);
                        if k > j {
                            bad = true;
                            cloned.swap(j, k);
                        }
                    }
                }
                vec_in = cloned;
            }
        }

        if bad {
            bad_middle_sum += vec_in[vec_in.len() / 2];
        } else {
            good_middle_sum += vec_in[vec_in.len() / 2];
        }
    }
    println!("Good Middle Values: {good_middle_sum}");
    println!("Bad Middle Values: {bad_middle_sum}");
}
