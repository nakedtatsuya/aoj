
fn markid(c: char) -> usize {
    for (mark, id) in "SHCD".chars().zip(0..4) {
        if c == mark {
            return id;
        }
    }
    panic!();
}

fn main(){
    
    let (n) = {
        let mut s = String::new(); // バッファを確保
        std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
        s.trim_right().to_owned(); // 改行コードが末尾にくっついてくるので削る
        let mut ws = s.split_whitespace(); // 空白区切りの単語に分解する
        let n: usize = ws.next().unwrap().parse().unwrap(); // イテレータから値を取り出して整数に
        (n)
    };
    let mut check_arr = vec![vec![false; 13]; 4];
    for _ in 0..n {
        let mut s = String::new(); // バッファを確保
        std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
        let mut ws = s.split_whitespace();
        let mark: char = ws.next().unwrap().chars().next().unwrap();
        let num: usize = ws.next().unwrap().parse().unwrap();

        let id = markid(mark);
        check_arr[id][num-1] = true;
    }
    
    for (mark, id) in "SHCD".chars().zip(0..4) {
        for num in 0..13 {
            if !check_arr[id][num] {
                println!("{} {}", mark, num + 1);
            }
        }
    }
}





