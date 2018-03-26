fn main() {
    let vec = read_line();

    // 0 0 1 2
    let sx : i32 = vec[0];
    let sy : i32 = vec[1];
    let tx : i32 = vec[2];
    let ty : i32 = vec[3];

    // 1 s-->t
    // 2 t-->s
    {
        // go ahead
        let h = (sy - ty).abs();
        let w = (sx - tx).abs();
        for _ in 0..h {
            print!("U");
        }
        for _ in 0..w {
            print!("R");
        }
        
        // go back
        for _ in 0..h {
            print!("D");
        }
        for _ in 0..w {
            print!("L");
        }
    }

    // 3 s-->t
    {
        let sx = sx - 1;
        let ty = ty + 1;
        let h = (sy - ty).abs();
        let w = (sx - tx).abs();
        print!("L");
        for _ in 0..h {
            print!("U");
        }
        for _ in 0..w {
            print!("R");
        }
        print!("D");
    }

    // 4 t-->s
    {
        let sy = sy - 1;
        let tx = tx + 1;

        let h = (sy - ty).abs();
        let w = (sx - tx).abs();
        print!("R");
        for _ in 0..h {
            print!("D");
        }
        for _ in 0..w {
            print!("L");
        }
        print!("U");
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

