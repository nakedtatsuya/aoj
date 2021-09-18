use std::io::*;
use std::str::FromStr;

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();
    let k: usize = sc.next();
    let mut vec: Vec<usize> = Vec::new();
    for _ in 0..n {
        vec.push(sc.next());
    }

    let mut base: usize = *vec.iter().max().unwrap();
    let mut max: usize = vec.iter().sum();


    while base < max {
        let mid = (base + max) / 2;

        let i = check(n, k, mid, &vec);
        if i < n {
            base = mid + 1;
        } else {
            max = mid;
        }
    }

    println!("{}", max);

}

fn check(n: usize, k: usize, p: usize, ws: &Vec<usize>) -> usize {
    let mut i = 0;
    'a: for _ in 0..k {
        let mut s = 0;
        while s + ws[i] <= p {
            s += ws[i];
            i += 1;
            if i == n {
                break 'a;
            }
        }
    }
    return i;
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


