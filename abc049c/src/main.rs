/*
dream dreamer erase eraser

AtCoderとRustのバージョンがあってない。.get使えない
*/
fn main() {
    // let mut org_s = String::from("erasedream");
    let mut org_s = read_line_str().replace("\n", "");
    let words = ["dreamer", "eraser", "dream", "erase"];
    let mut before_length = org_s.len();
    loop {
        for i in 0..words.len() {
            let mut s = org_s.clone();
            let length = words[i].len();
            if s.len() < length {
                continue;
            }

            let tail = s.get((s.len() - length)..s.len()).unwrap();
            let head = s.get(0..(s.len() - length)).unwrap();
            if tail == words[i] {
                org_s = String::from(head);
                break;
            }
        }
        if before_length == org_s.len() {
            println!("NO");
            return;
        }
        if org_s.len() <= 0 {
            println!("YES");
            return;
        }

        before_length = org_s.len();
    }

    println!("YES");
}

fn read_line_str() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    return s;
}