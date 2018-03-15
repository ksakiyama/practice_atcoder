fn main() {
    let v = read_line();
    let n = v[0];
    let target = v[1];
    // let n = 1000;
    // let target = 1234000;

    for nx in 0..n+1 {
        for ny in 0..(n+1-nx) {
            let nz = n - nx - ny;
            let res = nx * 10000 + ny * 5000 + nz * 1000;
            if res == target {
                println!("{} {} {}", nx, ny, nz);
                return;
            }
        }
    }

    println!("-1 -1 -1");
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