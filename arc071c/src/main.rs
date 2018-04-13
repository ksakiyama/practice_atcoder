use std::collections::HashMap;
use std::cmp;

fn main() {
    const INF : i32 = 10000;
    let ascii_lower = String::from("abcdefghijklmnopqrstuvwxyz");

    let mut vec = vec![];

    let n = read();
    for _ in 0..n {
        let mut map = HashMap::new();
        let s = read_line_str();
        for c in s.chars() {
            if map.contains_key(&c) {
                *map.get_mut(&c).unwrap() += 1;
            } else {
                map.insert(c, 1);
            }
        }
        vec.push(map);
    }

    let mut table = HashMap::new();

    for c in ascii_lower.chars() {
        let mut cnt = INF;
        for map in &vec {
            if map.contains_key(&c) {
                cnt = cmp::min(*map.get(&c).unwrap(), cnt);
            } else {
                cnt = 0;
            }
        }
        if cnt >= 1 {
            table.insert(c, cnt);
        }
    }

    let mut ans = String::from("");
    for c in ascii_lower.chars() {
        if table.contains_key(&c) {
            for _ in 0..*table.get(&c).unwrap() {
                ans.push(c);
            }
        }
    }

    // println!("{:?}", table);
    println!("{}", ans);
}

fn read() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_line_str() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    return s;
}