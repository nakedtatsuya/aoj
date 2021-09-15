
fn main(){

    loop {
        let (n, m, l) = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
            s.trim_right().to_owned(); // 改行コードが末尾にくっついてくるので削る
            let mut ws = s.split_whitespace(); // 空白区切りの単語に分解する
            let n: i64 = ws.next().unwrap().parse().unwrap(); // イテレータから値を取り出して整数に
            let m: String = String::from(ws.next().unwrap());
            let l: i64 = ws.next().unwrap().parse().unwrap(); // イテレータから値を取り出して整数に
            (n, m, l)
        };
        if m == "?" {
            break;
        } else if m == "+" {
            println!("{}", n+l);
        } else if m == "-" {
            println!("{}", n-l);
        } else if m == "*" {
            println!("{}", n*l);
        } else if m == "/" {
            println!("{}", n/l);
        }
    }
}