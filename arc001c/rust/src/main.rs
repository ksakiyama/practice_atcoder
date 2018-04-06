use std::io;

const N: usize = 8;

fn main() {
    let mut board = vec![false; N * N];
    for i in 0..N {
        let s = read_line();
        for (j, e) in s.chars().enumerate() {
            board[i * N + j] = if e == '.' { false } else { true }
        }
    }

    if dfs(0, 8, &mut board) {
        for y in 0..N {
            for x in 0..N {
                if board[y * N + x] {
                    print!("Q");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    } else {
        println!("No Answer");
    }
}

fn convert(pos: usize) -> (usize, usize) {
    let x = pos % N;
    let y = pos / N;
    return (x as usize, y as usize);
}

fn check_border(x: i64, y: i64) -> bool {
    let n = N as i64;
    if x < 0 || x > n - 1 || y < 0 || y > n - 1 {
        return false;
    }
    return true;
}

fn check_safe(x: i64, y: i64, board: &mut Vec<bool>) -> bool {
    for dx in -1..2 as i64 {
        for dy in -1..2 as i64 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let mut step = 1;
            loop {
                let nx = x + dx * step;
                let ny = y + dy * step;
                if !check_border(nx, ny) {
                    break;
                } else if board[ny as usize * N + nx as usize] {
                    return false;
                }
                step += 1;
            }
        }
    }
    return true;
}

fn dfs(pos: usize, n: usize, board: &mut Vec<bool>) -> bool {
    if n == 0 {
        return true;
    }
    if pos as usize == N * N {
        return false;
    }

    let (x, y) = convert(pos);
    if board[pos as usize] {
        if check_safe(x as i64, y as i64, board) {
            if dfs(pos + 1, n - 1, board) {
                return true;
            }
        }
    } else {
        if check_safe(x as i64, y as i64, board) {
            board[pos] = true;
            if dfs(pos + 1, n - 1, board) {
                return true;
            }
            board[pos] = false;
        }
        if dfs(pos + 1, n, board) {
            return true;
        }
    }

    false
}

fn read_line() -> String {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
