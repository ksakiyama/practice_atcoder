const NMAX : usize = 8;

fn main() {
    let vec = read_line();
    let n = vec[0];
    let m = vec[1];

    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..m {
        let vec = read_line();
        a.push(vec[0] - 1);
        b.push(vec[1] - 1);
    }

    let mut graph = vec![vec![false; NMAX]; NMAX];

    // 移動できるポイントにフラグを立てる
    for i in 0..m as usize {
        graph[a[i] as usize][b[i] as usize] = true;
        graph[b[i] as usize][a[i] as usize] = true;
    }

    let mut visited = vec![false; NMAX];
    visited[0] = true;

    let ans = dfs(0, n as usize, &mut visited, &mut graph);
    println!("{}", ans);
}

fn dfs(v : usize, n : usize, visited : &mut Vec<bool>, graph : &mut Vec<Vec<bool>>) -> i32 {
    let mut all_visited = true;
    for i in &*visited { // まじで！？
        if *i {
            all_visited = false;
        }
    }

    if all_visited {
        return 1;
    }

    let mut ret = 0;
    for i in 0..n as usize {
        if !graph[v][i] {
            continue;
        } else if visited[i] {
            continue;
        }

        visited[i] = true;
        ret += dfs(i, n, visited, graph);
        visited[i] = false;
    }

    return ret;
}

fn read_line() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let v = s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v;
}
