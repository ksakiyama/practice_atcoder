use std::collections::HashMap;

fn main() {
    let n = read();
    let mut v = Vec::new();
    for _ in 0..n {
        let s = read_line_str();
        v.push(s);
    }

    let mut m : usize = 0;
    let mut a : usize = 0;
    let mut r : usize = 0;
    let mut c : usize = 0;
    let mut h : usize = 0;
    for i in &v {
        let s = i.chars().nth(0).unwrap();
        if s == 'M' {
            m += 1;
        }
        if s == 'A' {
            a += 1;
        }
        if s == 'R' {
            r += 1;
        }
        if s == 'C' {
            c += 1;
        }
        if s == 'H' {
            h += 1;
        }
    }

    let P : [usize; 10] = [0, 0, 0, 0, 0, 0, 1, 1, 1, 2];
    let Q : [usize; 10] = [1, 1, 1, 2, 2, 3, 2, 2, 3, 3];
    let R : [usize; 10] = [2, 3, 4, 3, 4, 4, 3, 4, 4, 4];

    let mut D : [usize; 5] = [0; 5];
    D[0] = m;
    D[1] = a;
    D[2] = r;
    D[3] = c;
    D[4] = h;

    let mut res : usize = 0;
    for d in 0..10 {
        res += D [ P [ d ]]* D [ Q [ d ]]* D [ R [ d ]];
    }

    println!("{}", res);
}

fn read() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_line_str() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    return s;
}
