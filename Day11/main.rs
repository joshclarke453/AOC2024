use std::collections::HashMap;

fn blink(num_hash_map: HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut res: HashMap<i64, i64> = HashMap::new();
    for a in num_hash_map.clone().keys() {
        match a {
            0 => {
                match &res {
                    _x if (res.contains_key(&1)) => {
                        res.insert(1, num_hash_map.get(a).unwrap() + res.get(&1).unwrap());
                    }
                    _ => {
                        res.insert(1, *num_hash_map.get(a).unwrap());
                    }
                }
            }
            a if (a.to_string().len() % 2 == 0) => {
                let split = a.to_string();

                let fh: i64 = (split[..(split.len()/2)]).parse().unwrap();
                match &res {
                    _x if (res.contains_key(&fh)) => {
                        res.insert(fh, num_hash_map.get(a).unwrap() + res.get(&fh).unwrap());
                    }
                    _ => {
                        res.insert(fh, *num_hash_map.get(a).unwrap());
                    }
                }

                let sh: i64 = (split[(split.len()/2)..]).parse().unwrap();
                match &res {
                    _x if (res.contains_key(&sh)) => {
                        res.insert(sh, num_hash_map.get(a).unwrap() + res.get(&sh).unwrap());
                    }
                    _ => {
                        res.insert(sh, *num_hash_map.get(a).unwrap());
                    }
                }
            }
            _ => {
                let temp = a * 2024;
                match temp {
                    _x if (res.contains_key(&temp)) => {
                        res.insert(temp, num_hash_map.get(&a).unwrap() + res.get(&temp).unwrap());
                    }
                    _ => {
                        res.insert(temp, *num_hash_map.get(&a).unwrap());
                    }
                }
            }
        }
    }
    return res;
}

fn count_dict(num_hash_map: HashMap<i64, i64>) -> i64{
    let mut count = 0;
    for a in num_hash_map.clone().keys() {
        count += num_hash_map.get(a).unwrap();
    }
    return count;
}
fn main() {
    let mut res: HashMap<i64, i64> = HashMap::new();
    res.insert(5688, 1);
    res.insert(62084, 1);
    res.insert(2, 1);
    res.insert(3248809, 1);
    res.insert(179, 1);
    res.insert(79, 1);
    res.insert(0, 1);
    res.insert(172169, 1);

    for _x in 1..26 {
        res = blink(res.clone());
    }
    println!("25: {}", count_dict(res.clone()));

    for _x in 26..76 {
        res = blink(res.clone());
    }
    println!("75: {}", count_dict(res.clone()));
}