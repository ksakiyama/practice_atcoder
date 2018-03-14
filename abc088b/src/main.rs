/*
2
3 1

4
20 18 2 18
*/
fn main() {
    let n = read();
    let mut vec = read_line();
    vec.sort_by(|a, b| b.cmp(a));

    let mut alice = 0;
    let mut bob = 0;
    for i in 0..n as usize {
        if i % 2 == 0 {
            alice += vec[i];
        } else {
            bob += vec[i];
        }
    }

    println!("{}", alice - bob);
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