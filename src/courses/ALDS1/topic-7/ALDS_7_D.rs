use std::str::FromStr;
use std::{io::*, vec};

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();
    let mut pre_o: Vec<usize> = vec![0; n];
    let mut in_o: Vec<usize> = vec![0; n];
    let mut pos_o: Vec<usize> = vec![];
    for i in 0..n {
        pre_o[i] = sc.next();
    }

    for i in 0..n {
        in_o[i] = sc.next();
    }

    reconstruction(&pre_o, &in_o, &mut pos_o);

    for (i, val) in pos_o.iter().enumerate() {
        if i > 0 { print!(" "); }
        print!("{}", val);
    }
    print!("\n");
}

fn reconstruction(pre_o: &[usize], in_o: &[usize], pos_o: &mut Vec<usize>) {
    if in_o.is_empty() {
        return;
    }
    let r = pre_o[0];
    let mid = in_o.iter().position(|x| *x == r ).unwrap();
    reconstruction(&pre_o[1..], &in_o[0..mid], pos_o);
    reconstruction(&pre_o[mid + 1..], &in_o[mid + 1..], pos_o);
    pos_o.push(r);
    
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

