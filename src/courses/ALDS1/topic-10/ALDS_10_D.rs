use std::io::*;
use std::str::FromStr;
fn main(){
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    let n: usize = sc.next();
    let X: Vec<f64> = (0..n).map(|_| sc.next()).collect();
    let Y: Vec<f64> = (0..n).map(|_| sc.next()).collect();

    let mut p1 = 0.;
    let mut p2 = 0.;
    let mut p3 = 0.;
    let mut oo = 0.;
    for i in 0..n {
        let abs = (X[i] - Y[i]).abs();
        p1 += abs;
        p2 += abs.powi(2);
        p3 += abs.powi(3);
        if oo < abs {
            oo = abs;
        }
    }

    println!("{:.6}", p1);
    println!("{:.6}", p2.sqrt());
    println!("{:.6}", p3.cbrt());
    println!("{:.6}", oo);
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













