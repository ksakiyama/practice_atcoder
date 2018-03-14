/*
20 2 5
*/
fn main() {
    let vec = read_line();
    let n = vec[0];
    let a = vec[1];
    let b = vec[2];
    let mut sum = 0;
    for i in 1..n + 1 {
        let n5 = i / 10000;
        let n4 = i % 10000 / 1000;
        let n3 = i % 1000 / 100;
        let n2 = i % 100 / 10;
        let n1 = i % 10;

        let res = n1 + n2 + n3 + n4 + n5;
        if (res >= a) && (b >= res) {
            // println!("==debug====");
            // println!("i={}", i);
            // println!("res={}", res);
            // println!("{},{},{},{},{}", n5, n4, n3, n2, n1);
            sum += i;
        }
    }
    println!("{}", sum);
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