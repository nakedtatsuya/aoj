fn main() {
    let n: u32 = {
        let mut s = String::new(); // バッファを確保
        std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
        s.trim_right().to_owned().parse().unwrap() // 改行コードが末尾にくっついてくるので削る
    };
    let h = n / 3600;
    let m = n % 3600 / 60;
    let s = n % 3600 % 60;
    println!("{}:{}:{}", h, m, s);
}
