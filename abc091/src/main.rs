#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    t: String
}

fn main() {
    let n = read();
    let mut vec = Vec::new();

    for _ in 0..n {
        let v = read_line();
        vec.push( Point{x: v[0], y: v[1], t: "red".to_string()} )
    }
    for _ in 0..n {
        let v = read_line();
        vec.push( Point{x: v[0], y: v[1], t: "blue".to_string()} )
    }

    // Rule
    // red.x < blue.x
    // and
    // red.y < blue.y
    vec.sort_by(|a, b| a.x.cmp(&b.x));

    let mut cnt = 0;
    loop {
        for i in 0..vec.len() as usize {
            // let mut red;
            if vec[i].t == "blue" {
                let mut max_red_y = -1;
                let mut max_red_idx : usize = std::usize::MAX;
                for j in 0..i {
                    if vec[j].t == "red" {
                        if vec[i].x > vec[j].x && vec[i].y > vec[j].y {
                            if max_red_y < vec[j].y {
                                max_red_y = vec[j].y;
                                max_red_idx = j;
                            }
                        }
                    }
                }
                if max_red_idx == std::usize::MAX {
                    // 条件を満たす赤点はなかった。青だけ消す
                    vec.remove(i);
                    break;
                } else {
                    vec.remove(max_red_idx);
                    vec.remove(i - 1);
                    cnt += 1;
                    break;
                }
            }
        }
        let mut exist_blue = false;
        for i in 0..vec.len() {
            if vec[i].t == "blue" {
                exist_blue = true;
                break;
            }
        }
        if !exist_blue {
            break;
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