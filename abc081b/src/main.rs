/*
3
[8, 12, 40];

6
[382253568, 723152896, 37802240, 379425024, 404894720, 471526144]
*/
fn main() {
    let n = read();
    let mut vec = read_line();

    let mut cnt = std::u32::MAX;
    for i in 0..n as usize {
        let mut t_cnt = 0;
        while vec[i] % 2 == 0 && vec[i] != 0 {
            vec[i] /= 2;
            t_cnt += 1;
        }
        cnt = std::cmp::min(t_cnt, cnt);
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