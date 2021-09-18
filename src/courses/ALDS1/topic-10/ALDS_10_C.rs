use std::str::FromStr;
use std::{io::*};
use std::cmp;

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();
    
    for _ in 0..n {
        let X: String = sc.next();
        let Y: String = sc.next();

        let count = longest_common_subsequence(X, Y);
        println!("{}", count);
    }

}

fn longest_common_subsequence(X: String, Y: String) -> usize {
    let mut c = [[0;1001]; 1001];
    let m = X.len();
    let n = Y.len();
    let mut maxl = 0;

    for i in 1..=m {
        for j in 1..=n {
            if X.get(i-1..i) == Y.get(j-1..j) {
                c[i][j] = c[i-1][j-1] + 1;
            } else {
                c[i][j] = cmp::max(c[i][j-1], c[i-1][j]);
            }
            maxl = cmp::max(maxl,  c[i][j]);
        }
    }
    maxl
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


