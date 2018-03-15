/*
4
10
8
8
6
*/
fn main() {
    let n = read();
    let mut d = Vec::new();
    for i in 0..n {
        d.push(read());
    }

    d.sort();
    d.dedup();

    println!("{}", d.len());
}

fn read() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
