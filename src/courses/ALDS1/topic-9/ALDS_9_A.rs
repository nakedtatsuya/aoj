use std::str::FromStr;
use std::{io::*};

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();
    let mut A: Vec<isize> = vec![0; n+1];
    for i in 1..=n {
        A[i] = sc.next();
    }

    for i in 1..=n {

        print!("node {}: key = {}, ", i, A[i]);
        if i / 2 > 0 {
            print!("parent key = {}, ", A[i / 2]);
        }
        if i * 2 <= n {
            print!("left key = {}, ", A[i * 2]);
        }
        if i * 2 + 1 <= n {
            print!("right key = {}, ", A[i * 2 + 1]);
        }
        print!("\n");
    }
}

fn max_heapify(A: &mut Vec<isize>, i: usize) {
    let l = i * 2;
    let r = i * 2 + 1;

    // if l <= A.len() && A[l-1] > 
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


