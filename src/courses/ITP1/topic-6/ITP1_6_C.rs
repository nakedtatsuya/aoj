fn main(){
    let (n) = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_right().to_owned();
        let mut ws = s.split_whitespace();
        let n: usize = ws.next().unwrap().parse().unwrap();
        (n)
    };
    
    // 入れ物
    let mut aparts = vec![vec![vec![0; 10]; 3]; 4]; // 4B 3F 10R
    
    for _ in 0..n {
        let mut s = String::new(); // バッファを確保
        std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
        let mut ws = s.split_whitespace();
        let b: usize = ws.next().unwrap().parse().unwrap();
        let f: usize = ws.next().unwrap().parse().unwrap();
        let r: usize = ws.next().unwrap().parse().unwrap();
        let v: isize = ws.next().unwrap().parse().unwrap();
        aparts[b-1][f-1][r-1] += v;
    }
    
    
    // 出力
    for b in 0..4 {
        if b != 0 {
            println!("{}", "#".repeat(20));
        }
        for f in 0..3 {
            for r in 0..10 {
                print!(" {}", aparts[b][f][r]);
            }
            println!();
        }
    }
}







