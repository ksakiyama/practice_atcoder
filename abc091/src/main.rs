use std::collections::HashMap;

fn main() {
    let n = read();
    let mut blue_s = Vec::new();
    for _ in 0..n {
        blue_s.push(read_line_str());
    }

    let m = read();
    let mut red_t = Vec::new();
    for _ in 0..m {
        red_t.push(read_line_str());
    }

    let mut hmap = HashMap::new();
    for e in &blue_s {
        if hmap.contains_key(e) {
            // **このパタンもあり**
            // if let Some(x) = hmap.get_mut(e) {
            //     *x = *x + 1;
            // }

            // **これもいける**
            // let x = hmap.get_mut(e).unwrap();
            // *x += 1;

            // **これもいけるのか**
            *(hmap.get_mut(e).unwrap()) += 1;
        } else {
            hmap.insert(e, 1);
        }
    }

    for e in &red_t {
        if hmap.contains_key(e) {
            if let Some(x) = hmap.get_mut(e) {
                *x = *x - 1;
            }
        } else {
            hmap.insert(e, -1);
        }
    }

    // &i32などは、*をつけるとその値になる
    let mut price = 0;
    for val in hmap.values() {
        if price < *val {
            price = *val;
        }
        // println!("{} {}", key, val);
    }

    println!("{}", price);
}

fn read() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_line_str() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    return s.replace("\n", "");
}
