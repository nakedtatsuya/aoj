use std::io::*;
use std::str::FromStr;
fn main(){
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    let x1: f64 = sc.next();
    let y1: f64 = sc.next();
    let x2: f64 = sc.next();
    let y2: f64 = sc.next();
    
    // let res = (1f64 - 0f64).pow(2) + (1f64 - 0f64).pow(2);
    let x = (x2 - x1).powf(2.0);
    let y = (y2 - y1).powf(2.0);

    // let res: f64 = res.parse::<f64>().unwrap();

    println!("{}", (x + y).sqrt());
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













