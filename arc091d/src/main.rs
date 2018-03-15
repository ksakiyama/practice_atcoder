fn main() {
    // let n = 5;
    // let k = 2;
    // let n = 10;
    // let k = 1;
    let n = 31415;
    let k = 9265;
    // let vec = read_line();
    // let n = vec[0];
    // let k = vec[1];

    // if k == 0 {
    //     println!("{}", n * n);
    //     return;
    // }

    let mut cnt = 0;
    for b in 1..(n+1) {
        let p = n / b;
        let r = n % b;
        let sum1 = std::cmp::max(0, b - k) * p;
        let sum2 = std::cmp::max(0, r - k + 1);
        cnt += sum1 + sum2;
    }

    println!("{}", cnt);
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