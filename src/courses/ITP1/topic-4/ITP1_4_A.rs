fn main(){
   let (n, m) = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
        s.trim_right().to_owned(); // 改行コードが末尾にくっついてくるので削る
        let mut ws = s.split_whitespace(); // 空白区切りの単語に分解する
        let n: u32 = ws.next().unwrap().parse().unwrap(); // イテレータから値を取り出して整数に
        let m: u32 = ws.next().unwrap().parse().unwrap();
        (n, m)
    };
    let res = n as f64 / m as f64;
    println!("{} {} {:.5}", n / m, n % m, res);
}
