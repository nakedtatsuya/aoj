fn main(){
   let (r) = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
        s.trim_right().to_owned(); // 改行コードが末尾にくっついてくるので削る
        let mut ws = s.split_whitespace(); // 空白区切りの単語に分解する
        let r: f64 = ws.next().unwrap().parse().unwrap(); // イテレータから値を取り出して整数に
        (r)
    };
    println!("{:.5} {:.5}", r*r*std::f64::consts::PI, 2_f64*r*std::f64::consts::PI);
}
