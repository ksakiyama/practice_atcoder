use std::collections::HashMap;

fn main() {
    let n = read();

    let mut map: HashMap<i64, i64> = HashMap::new();
    for i in 1..n + 1 {
        trial_division_sqrt(i, &mut map);
    }

    // for (k, v) in &map {
    //     println!("{}, {}", k, v);
    // }
    let mut ans: i64 = 1;
    for v in map.values() {
        // println!("{}", *v);
        ans *= *v + 1;
        ans %= 1000000007;
    }

    println!("{}", ans);
}

fn trial_division_sqrt(n: i64, map: &mut HashMap<i64, i64>) {
    let mut n = n;
    let limit = ((n as f64).sqrt() + 2.0) as i64;

    for i in 2..limit {
        while n % i == 0 {
            n /= i;
            if map.contains_key(&i) {
                *map.get_mut(&i).unwrap() += 1;
            } else {
                map.insert(i, 1);
            }
        }
    }
    if n > 1 {
        if map.contains_key(&n) {
            *map.get_mut(&n).unwrap() += 1;
        } else {
            map.insert(n, 1);
        }
    }
}

fn read() -> i64 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
