fn main() {
    let mut n = read();
    let mut vec = read_line();
    n += 2;
    vec.insert(0, 0);
    vec.push(0);

    let mut total = 0;
    for i in 1..n as usize {
        let dif = (vec[i - 1] - vec[i]).abs();
        total += dif;
    }

    for i in 1..(n - 1) as usize {
        let left = vec[i - 1];
        let center = vec[i];
        let right = vec[i + 1];

        if left <= center && center <= right {
            println!("{}", total);
        } else if left >= center && center >= right {
            println!("{}", total);
        } else {
            let dif1 = (left - center).abs();
            let dif2 = (center - right).abs();
            let dif3 = (left - right).abs();
            println!("{}", total - dif1 - dif2 + dif3);
        }
    }

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
    return v;
}
