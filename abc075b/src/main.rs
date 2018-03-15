/*
3 5
.....
.#.#.
.....
*/
fn main() {
    // let h : i32 = 3;
    // let w : i32 = 5;
    // let mut lines = Vec::new();
    // lines.push(".....");
    // lines.push(".#.#.");
    // lines.push(".....");
    let tmp = read_line();
    let h = tmp[0];
    let w = tmp[1];
    let mut lines = Vec::new();
    for _ in 0..h {
        let s = read_line_str();
        lines.push(s);
    }

    let mut cells : Vec<Vec<i32>> = Vec::new();
    for i in 0..h {
        let mut vec = Vec::new();
        for j in lines[i as usize].chars() {
            if j == '.' {
                vec.push(0);
            } else {
                vec.push(-1);
            }
        }
        cells.push(vec);
    }

    for y in 0..h as i32 {
        for x in 0..w as i32 {
            if cells[y as usize][x as usize] == -1 {
                // search
                for dy in (-1 as i32)..2 {
                    for dx in (-1 as i32)..2 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = x + dx;
                        let ny = y + dy;
                        if nx < 0 || ny < 0 || nx >= w || ny >= h {
                            continue;
                        }
                        if cells[ny as usize][nx as usize] == -1 {
                            continue;
                        }
                        cells[ny as usize][nx as usize] += 1;
                    }
                }
            }
        }
    }

    for vec in cells {
        for v in vec {
            if v == -1 {
                print!("#");
            } else {
                print!("{}", v);
            }
        }
        println!("");
    }
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

fn read_line_str() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    return s.replace("\n", "");
}