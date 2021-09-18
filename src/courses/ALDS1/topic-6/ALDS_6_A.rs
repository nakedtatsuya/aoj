use std::{io::*, vec};
use std::str::FromStr;

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();

    let mut A: Vec<usize> = Vec::new();
    for _ in 0..n {
        A.push(sc.next());
    }

    let max: usize = *A.iter().max().unwrap();

    let B = couting_sort(A, max);
    for v in 0..B.len()-1 {
        print!("{} ", B[v]);
    }
    println!("{}", B[B.len()-1]);
}

fn couting_sort(A: Vec<usize>, k: usize) -> Vec<usize> {
    let mut C: Vec<usize> = vec![0;k+1];
    let mut B: Vec<usize> = vec![0;A.len()];
    // C[i] に i の出現数を記録する
    for j in 0..A.len() {
        C[A[j]] += 1;
    }

    // C[i] に i 以下の数の出現数を記録する
    for i in 1..k+1 {
        C[i] = C[i] + C[i-1];
    }

    for j in (0..A.len()).rev() {
        B[C[A[j]]-1] = A[j];
        C[A[j]] -= 1;
    }
    B
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
            count += l.len()-1-i;
        }
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




