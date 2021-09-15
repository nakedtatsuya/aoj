use std::cmp::Ordering;
fn main() {
    let (n, m, l) = {
        let mut s = String::new(); // バッファを確保
        std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
        s.trim_right().to_owned(); // 改行コードが末尾にくっついてくるので削る
        let mut ws = s.split_whitespace(); // 空白区切りの単語に分解する
        let n: i32 = ws.next().unwrap().parse().unwrap(); // イテレータから値を取り出して整数に
        let m: i32 = ws.next().unwrap().parse().unwrap();
        let l: i32 = ws.next().unwrap().parse().unwrap();
        (n, m, l)
    };
    if n < m && m < l {println!("Yes");} else {println!("No");}
}
