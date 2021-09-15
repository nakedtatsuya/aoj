fn main(){
    loop {
        let (m, f, r) = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_right().to_owned();
            let mut ws = s.split_whitespace();
            let m: isize = ws.next().unwrap().parse().unwrap();
            let f: isize = ws.next().unwrap().parse().unwrap();
            let r: isize = ws.next().unwrap().parse().unwrap();
            (m, f, r)
        };
       
        if let Some(result) = check_result(m, f, r) {
            println!("{}", result);
        } else {
            break;
        }
        
    }
}

fn check_result(m: isize, f: isize, r: isize) -> Option<char> {
    match (m, f, r) {
        (-1, -1, -1) => None,
        (-1, _, _) => Some('F'),
        (_, -1, _) => Some('F'),
        _ if m+f >= 80 => Some('A'),
        _ if m+f >= 65 => Some('B'),
        _ if m+f >= 50 => Some('C'),
        _ if m+f >= 30 && r >= 50 => Some('C'),
        _ if m+f >= 30 => Some('D'),
        _ => Some('F'),
    }
}
