use std::io::*;
use std::str::FromStr;
use std::collections::VecDeque;

fn main(){

    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();

    let mut vec: VecDeque<usize> = VecDeque::new();
    for _ in 0..n {
        let ord: String = sc.next();

        match ord.as_str() {
            "insert" => {
                let v: usize = sc.next();
                vec.push_front(v);
            },
            "delete" => {
                let v: usize = sc.next();
                let index = vec.iter().position(|x| *x == v);
                if let Some(index) = index {
                    vec.remove(index);
                }
            },
            "deleteFirst" => {
                vec.pop_front();
            },
            "deleteLast" => {
                vec.pop_back();
            },
            _ => {},
        }
    }

    while !vec.is_empty() {
        let v = vec.pop_front().unwrap();
        if !vec.is_empty() {
            print!("{} ", v);
        } else {
            println!("{}", v);
        }
    }
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


