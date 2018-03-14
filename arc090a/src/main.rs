use std::cmp;

struct Solver {
    w: i32,
    cells: Vec<Vec<i32>>,
}

impl Solver {
    fn new() -> Solver {
        let w = read();
        let h = 2;
        Solver {
            w: w,
            cells: read_lines(h),
        }
    }

    fn run(&self) {
        let mut best = 0;

        // 最も良いルートを探索
        for i in 0..self.w as usize {
            let mut value = 0;
            let mut y = 0;
            for x in 0..self.w as usize {
                if i == x {
                    y = 1;
                    value = value + self.cells[0][x];
                }
                value = value + self.cells[y][x];
            }
            best = cmp::max(best, value);
        }

        println!("{}", best);
    }

    fn show(&self) {
        for row in &self.cells {
            for i in row {
                print!("{} ", i);
            }
            println!("");
        }
    }
}

fn read() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_lines(h: i32) -> Vec<Vec<i32>> {
    let mut v2 = Vec::new();
    for _ in 0..h {
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

fn main() {
    let solver = Solver::new();
    // solver.show()
    solver.run();
}
