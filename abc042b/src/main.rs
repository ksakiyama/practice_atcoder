fn main() {
    let nm = read_line();
    let mut vec = vec![];
    for _ in 0..nm[0] {
        let s = read_line_str().replace("\n", "");
        vec.push(s);
    }

    vec.sort();
    for i in vec {
        print!("{}", i);
    }
    println!("");
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

fn read_line_str() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    return s;
}