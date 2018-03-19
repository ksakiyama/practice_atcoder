fn main() {
    // cells[j][i] == a[i] + b[j]
    let c = read_lines(3);
    let mut a = vec![0, 0, 0];
    let mut b = vec![0, 0, 0];

    a[0] = 0;
    b[0] = c[0][0] - a[0];
    b[1] = c[1][0] - a[0];
    b[2] = c[2][0] - a[0];
    a[1] = c[0][1] - b[0];
    a[2] = c[0][2] - b[0];

    for j in 0..3 {
        for i in 0..3 {
            if c[j][i] != a[i] + b[j] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}

fn read_lines(n: i32) -> Vec<Vec<i32>> {
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
