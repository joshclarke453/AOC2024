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
    let mut bad_count = 0;
    let mut good_middle_sum = 0;
    for line in input_lines {
        let split_in: Vec<&str> = line.split(",").collect();
        let mut vec_in: Vec<i32> = Vec::new();
        for x in split_in {
            vec_in.push(x.parse().unwrap());
        }

        let mut bad = false;
        for j in 0..vec_in.len() {
            if bad {
                break;
            }
            for k in 0..vec_in.len() {
                if bad {
                    break;
                }
                if j == k {
                    continue;
                }

                for r in &rules_vec {
                    if vec_in[j] == r[0] && vec_in[k] == r[1] {
                        if j > k {
                            bad = true;
                            break;
                        };
                    } else if vec_in[j] == r[1] && vec_in[j] == r[0] {
                        if k > j {
                            bad = true;
                            break;
                        }
                    }
                }
            }
        }

        if bad {
            bad_count += 1;
        } else {
            good_middle_sum += vec_in[vec_in.len() / 2];
        }
    }
    println!("Bad Count: {bad_count}");
    println!("Good Middle Values: {good_middle_sum}");
}
