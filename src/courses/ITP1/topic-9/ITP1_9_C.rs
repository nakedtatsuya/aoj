use std::cmp::Ordering::*;
use std::io::*;
use std::str::FromStr;
fn main(){
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    
    
    let mut taro_score: u32 = 0;
    let mut hanako_score: u32 = 0;

    let n: u32 = sc.next();
    for _ in 0..n {
        let t: String = sc.next();
        let h: String = sc.next();
        match t.cmp(&h) {
            Greater => {
                taro_score += 3;
            }
            Equal => {
                taro_score += 1;
                hanako_score += 1;
            }
            Less => {
                hanako_score += 3;
            }
        }
    }
    println!("{} {}", taro_score, hanako_score);
    
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











