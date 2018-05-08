fn read() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn isprime(p: i32) -> bool {
    if p == 1 {
        return false;
    }
    for i in 2..p {
        if p % i == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut n = read();
    let mut ans = vec![];
    let mut i = 31;
    while i < 55555 + 1 {
        if isprime(i) {
            ans.push(i.to_string());
            n -= 1;
        }
        if n == 0 {
            break;
        }
        i += 30;
    }

    println!("{}", ans.join(" "));
}
