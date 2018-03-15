/*
2
3 1 2
6 1 1
*/
fn main() {
    // let n = 2;
    // let vt = vec![0, 3, 6];
    // let vx = vec![0, 1, 1];
    // let vy = vec![0, 2, 1];
    let n = read();
    let vec = read_lines(n);
    let mut vt = vec![0];
    let mut vx = vec![0];
    let mut vy = vec![0];
    for v in vec {
        vt.push(v[0]);
        vx.push(v[1]);
        vy.push(v[2]);
    }

    let mut can_reach = true;

    for i in 1..(n + 1) as usize {
        let dt : i32 = vt[i] - vt[i - 1];
        let dx = ((vx[i] - vx[i - 1]) as i32).abs();
        let dy = ((vy[i] - vy[i - 1]) as i32).abs();
        let length = dx + dy;
        let div = (length - dt).abs();

        // debug
        // println!("{} {} {} {}", dt, dx, dy, div);

        if length > dt {
            can_reach = false;
        }

        if div % 2 == 1 {
            can_reach = false;
        }

        if !can_reach {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

fn read() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
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
