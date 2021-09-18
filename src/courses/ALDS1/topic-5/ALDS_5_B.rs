use std::{io::*, vec};
use std::str::FromStr;

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();

    let mut m_vec: Vec<i32> = Vec::new();
    for _ in 0..n {
        m_vec.push(sc.next());
    }

    let a = m_vec.len();

    let count = merge_sort(&mut m_vec, 0, a);
    print_vec(m_vec);
    println!("{}", count);
}

fn merge(a: &mut [i32], left: usize, mid: usize, right: usize) -> usize {
    let mut count = 0;
    let mut l = a[left..mid].to_vec();
    let mut r = a[mid..right].to_vec();
    // 番兵の挿入
    l.push(std::i32::MAX);
    r.push(std::i32::MAX);
    let mut i = 0;
    let mut j = 0;
    for k in left..right {
        if l[i] <= r[j] {
            a[k] = l[i];
            i += 1;
        } else {
            a[k] = r[j];
            j += 1;
        }
        count += 1;
    }
    count
}

fn merge_sort(a: &mut [i32], left: usize, right: usize) -> usize {
    let mut count = 0;
    if left + 1 < right {
        let mid = (left + right) / 2;
        count += merge_sort(a, left, mid);
        count += merge_sort(a, mid, right);
        count += merge(a, left, mid, right);
    }
    count
}

fn print_vec<T: std::fmt::Display>(v: Vec<T>) {
    for a in &v[0..v.len() - 1] {
        print!("{} ", a);
    }
    println!("{}", v[v.len() - 1]);
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



