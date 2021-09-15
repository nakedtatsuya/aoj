fn main() {
    let n: u32 = {
        let mut s = String::new(); // バッファを確保
        std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
        s.trim_right().to_owned().parse().unwrap() // 改行コードが末尾にくっついてくるので削る
    };
    println!("{}", n.pow(3));
}
