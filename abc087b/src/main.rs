/*
30
40
50
6000
*/
fn main() {
    let a = read();
    let b = read();
    let c = read();
    let x = read();

    let mut cnt = 0;
    for na in 0..a + 1 {
        for nb in 0..b + 1 {
            for nc in 0..c + 1 {
                let sum = na * 500 + nb * 100 + nc * 50;
                if sum == x {
                    cnt += 1;
                }
            }
        }
    }

    println!("{}", cnt);
}

fn read() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
