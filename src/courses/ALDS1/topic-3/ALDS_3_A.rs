use std::io::*;
use std::str::FromStr;

fn main(){

    let mut s = String::new(); // バッファを確保
    std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
    s.trim_right().to_owned(); // 改行コードが末尾にくっついてくるので削る
   
    let mut vec: Vec<isize> = Vec::new();
    for w in s.split_whitespace() {
        match w {
            "+" => {
                let right = vec.pop().unwrap();
                let left = vec.pop().unwrap();
                vec.push(left+right);
            },
            "-" => {
                let right = vec.pop().unwrap();
                let left = vec.pop().unwrap();
                vec.push(left-right);
            },
            "*" => {
                let right = vec.pop().unwrap();
                let left = vec.pop().unwrap();
                vec.push(left*right);
            },
            "/" => {
                let right = vec.pop().unwrap();
                let left = vec.pop().unwrap();
                vec.push(left/right);
            },
            _ => {
                vec.push(w.parse::<isize>().unwrap());
            },
        }
    }
   
    println!("{}", vec[0]);
  
}

/* ========== Scanner ========== */

struct Scanner<R: Read> {
    reader: R,
}

#[allow(dead_code)]
impl<R: Read> Scanner<R> {
    fn new(reader: R) -> Scanner<R> {
        Scanner { reader: reader }
    }

    fn read<T: FromStr>(&mut self) -> Option<T> {
        let token = self
            .reader
            .by_ref()
            .bytes()
            .map(|c| c.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        if token.is_empty() {
            None
        } else {
            token.parse::<T>().ok()
        }
    }

    fn next<T: FromStr>(&mut self) -> T {
        if let Some(s) = self.read() {
            s
        } else {
            writeln!(stderr(), "Terminated with EOF").unwrap();
            std::process::exit(0);
        }
    }

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next()).collect()
    }

    fn char(&mut self) -> char {
        self.next::<String>().chars().next().unwrap()
    }
}


