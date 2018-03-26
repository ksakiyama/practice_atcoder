use std::cmp::min;

fn main() {
    let vec = read_line();
    let n = vec[0];
    let m = vec[1];

    let vec = read_lines(m);
    let mut a = vec![];
    let mut b = vec![];
    let mut c = vec![];
    let mut dist = vec![vec![0; 100]; 100];    
    for v in vec {
        a.push(v[0] - 1);
        b.push(v[1] - 1);
        c.push(v[2]);
    }

    // すべての頂点について辞書をつくる
    for i in 0..n as usize {
        for j in 0..n as usize {
            if i == j {
                dist[i][j] = 0;
            } else {
                dist[i][j] = 100000000; // INF
            }
        }
    }

    // 辞書に移動距離を埋める
    for i in 0..m as usize {
        let idx_ai = a[i] as usize;
        let idx_bi = b[i] as usize;
        dist[idx_ai][idx_bi] = min(dist[idx_ai][idx_bi], c[i]);
        dist[idx_bi][idx_ai] = min(dist[idx_bi][idx_ai], c[i]);
    }

    for k in 0..n as usize {
        for i in 0..n as usize {
            for j in 0..n as usize {
                dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }

    // let mut ans = m;
    // for i in 0..m as usize {
    //     let mut shortest = false;
    //     for j in 0..n as usize {
    //         if dist[j][a[i] as usize] + c[i] == dist[j][b[i] as usize] {
    //             shortest = true;
    //         }
    //     }
    //     if shortest {
    //         ans -= 1;
    //     }
    // }
    let mut ans = 0;
    for i in 0..m as usize {
        if dist[a[i] as usize][b[i] as usize] < c[i] {
            ans += 1;
        }
    }

    println!("{}", ans);
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

fn read_lines(n : i32) -> Vec<Vec<i32>> {
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