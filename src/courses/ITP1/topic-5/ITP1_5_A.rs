fn main(){

    loop {
        let (h, w) = {
            let mut s = String::new(); // バッファを確保
            std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
            s.trim_right().to_owned(); // 改行コードが末尾にくっついてくるので削る
            let mut ws = s.split_whitespace(); // 空白区切りの単語に分解する
            let h: usize = ws.next().unwrap().parse().unwrap(); // イテレータから値を取り出して整数に
            let w: usize = ws.next().unwrap().parse().unwrap();
            (h, w)
        };
        
        if h == 0 && w == 0 {
            break;
        }
        
        for _ in 0..h {
            println!("{}", "#".repeat(w));
        }
        print!("\n");
    }
}

