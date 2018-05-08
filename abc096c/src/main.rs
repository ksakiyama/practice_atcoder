fn read_line() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let v = s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v;
}

fn read_string() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim_right().to_string()
}

fn main() {
    let vec = read_line();
    let h = vec[0];
    let w = vec[1];

    let mut cells = vec![];
    for _ in 0..h {
        let s = read_string();
        let mut vec = vec![];
        for c in s.chars() {
            vec.push(c);
        }
        cells.push(vec);
    }

    let directions = vec![[-1, 0], [0, 1], [1, 0], [0, -1]];

    for y in 0..h {
        for x in 0..w {
            if cells[y as usize][x as usize] == '#' {
                let mut drawable = false;
                for d in &directions {
                    let px = x + d[0];
                    let py = y + d[1];
                    if px < 0 || px >= w || py < 0 || py >= h {
                        continue;
                    }
                    if cells[py as usize][px as usize] == '#' {
                        drawable = true;
                        break;
                    }
                }
                if !drawable {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}
