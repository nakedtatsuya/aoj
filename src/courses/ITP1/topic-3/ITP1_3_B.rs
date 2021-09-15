fn main(){
    
    
    let mut count = 1;
    loop {
        let mut s = String::new(); // バッファを確保
        std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
        let n: i32 = s.trim().parse().unwrap();
        if n == 0 {
            break;
        }
        println!("Case {}: {}", count, n);
        count += 1;
    }
}

