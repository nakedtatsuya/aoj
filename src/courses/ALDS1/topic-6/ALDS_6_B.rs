use std::{io::*, vec};
use std::str::FromStr;

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();

    let mut A: Vec<usize> = vec![0; n];
    for i in 0..n {
        A[i] = sc.next();
    }
    let r = A.len() - 1;
    let q = partition(&mut A, 0, r);

    print!("{}", A[0]);
    for i in 1..n {
        if i == q {
            print!(" [{}]", A[i]);
        } else {
            print!(" {}", A[i]);
        }
    }
    print!("\n");
}

fn partition(A: &mut Vec<usize>, p: usize, r: usize) -> usize {
    let x = A[r];
    let mut i = p;
    for j in p..r {
        if A[j] <= x {
            A.swap(i, j);
            i += 1;
        }
    }
    A.swap(i, r);
    i
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




