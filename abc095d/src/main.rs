fn read_line() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let v = s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v;
}

fn main() {
    use std::cmp;

    let vec = read_line();
    let n = vec[0] as usize;
    let c = vec[1];
    let half_c = (c as f32 / 2.0).floor() as i32;

    let get_d = |a: i32, b: i32| -> i32 {
        let d = (a - b).abs();
        if d > half_c {
            return d - half_c;
        }
        d
    };

    let mut pos = vec![];
    let mut val = vec![];
    for _ in 0..n {
        let vec = read_line();
        pos.push(vec[0]);
        val.push(vec[1]);
    }

    let mut best_gain = 0;
    for y in 0..n {
        for x in 0..n {
            if x > y {
                continue;
            }

            let mut part1 = 0;
            let mut cost1 = 0;
            for i in 0..x {
                part1 += val[i];
                cost1 = pos[i];
            }

            let mut part2 = 0;
            let mut cost2 = c - pos[y];
            for i in y..n {
                part2 += val[i];
            }

            let gain = cmp::max(
                part1 + part2 - cost1 * 2 - cost2,
                part1 + part2 - cost1 - cost2 * 2,
            );

            if gain > best_gain {
                best_gain = gain;
            }
        }
    }

    println!("{}", best_gain);
}
