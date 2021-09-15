fn main(){
    let (n) = {
        let mut s = String::new(); // バッファを確保
        std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
        s.trim_right().to_owned(); // 改行コードが末尾にくっついてくるので削る
        let mut ws = s.split_whitespace(); // 空白区切りの単語に分解する
        let n: usize = ws.next().unwrap().parse().unwrap(); // イテレータから値を取り出して整数に
        (n)
    };
    for i in 1..(n+1) {
        if i % 3 == 0 {
            print!(" {}", i);
            continue;
        }
        let mut x = i;
        while x > 0 {
            if x % 10 == 3 {
                print!(" {}", i);
                break;
            } else {
                x /= 10;
            }
        }
    }
    println!();
}