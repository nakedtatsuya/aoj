use std::io::*;
use std::str::FromStr;
use std::cmp::Ordering::{self, Equal, Greater, Less};
use std::result::Result;

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();
    let mut a_n: Vec<usize> = Vec::new();
    for i in 0..n {
        a_n.push(sc.next());
    }
    a_n.sort();

    let t: usize = sc.next();
    let mut a_t: Vec<usize> = Vec::new();
    for _ in 0..t {
        let v: usize = sc.next();

        if let Ok(v) = binary_search(&a_n, v) {
            a_t.push(v);
        }
    } 

    println!("{}", a_t.len());

}

fn binary_search(arr: &Vec<usize>, t: usize) -> Result<usize, usize> {
    let mut size = arr.len();
    if size == 0 {
        return Err(0);
    }
    let mut base = 0;

    
    while size > 1 {
        let half = size / 2;
        let mid = half + base;
        let res = arr[mid].cmp(&t);
        if res == Equal { return Ok(t) }
        base = if res == Greater { base } else { mid };
        size -= half;
    }
    Err(0)
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


