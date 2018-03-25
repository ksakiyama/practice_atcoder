use std::collections::HashMap;

fn main() {
    let _ = read();
    let vec = read_line();
    
    let mut map : HashMap<i32, i32> = HashMap::new();

    for key in vec {
        if map.contains_key(&key) {
            let cnt = *map.get(&key).unwrap();
            map.insert(key, cnt + 1);
        } else {
            map.insert(key, 1);
        }
    }

    let mut cnt = 0;
    for (k, v) in map {
        if k > v {
            cnt += v;
        } else if k < v {
            cnt += v - k;
        }
    }

    println!("{}", cnt);
}

fn read() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_line() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let v = s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v
}