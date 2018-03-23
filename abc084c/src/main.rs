fn main() {
    let n = read();

    // 0:C 1:S 2:F
    let vec = read_lines(n - 1);
    for i in 0..n {
        let mut t = 0;
        for j in i..n-1 {
            if t < vec[j][1] {
                t = vec[j][1];
            } else if t % vec[j][2] == 0 {
                // do nothing
            } else {
                t = t + vec[j][2] - t % vec[j][2];
            }
            t = t + vec[j][0];
        }
        println!("{}", t);
    }
}

fn read() -> usize {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_lines(n: usize) -> Vec<Vec<i32>> {
    let mut v2 = Vec::new();
    for _ in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        let v = s.trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();
        v2.push(v);
    }
    v2
}
