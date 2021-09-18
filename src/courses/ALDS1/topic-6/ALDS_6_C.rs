use std::{io::*, vec};
use std::str::FromStr;

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();

    let mut A: Vec<(char, usize)> = vec![('A', 0); n];
    for i in 0..n {
        A[i] = (sc.next(), sc.next());
    }
    let r = A.len() - 1;
    let mut B = A.clone();
    quick_sort(&mut A, 0, r);
    merge_sort(&mut B, 0, r+1);

    let mut stable = true;

    for i in 0..n {
        if A[i].0 != B[i].0 {
            stable = false;
            break;
        }
    }

    if stable {
        println!("Stable");
    } else {
        println!("Not stable");
    }

    for i in 0..n {
        println!("{} {}", A[i].0, A[i].1);
    }
    

    // print!("{}", A[0]);
    // for i in 1..n {
    //     if i == q {
    //         print!(" [{}]", A[i]);
    //     } else {
    //         print!(" {}", A[i]);
    //     }
    // }
    // print!("\n");
}

fn quick_sort(A: &mut Vec<(char, usize)>, p: usize, r: usize) {
    if p < r {
        let q = partition(A, p, r);
        quick_sort(A, p, q-1);
        quick_sort(A, q+1, r);
    }
}

fn partition(A: &mut Vec<(char, usize)>, p: usize, r: usize) -> usize {
    let x = A[r].1;
    let mut i = p;
    for j in p..r {
        if A[j].1 <= x {
            A.swap(i, j);
            i += 1;
        }
    }
    A.swap(i, r);
    i
}

fn merge(a: &mut [(char, usize)], left: usize, mid: usize, right: usize) -> usize {
    let mut count = 0;
    let mut l = a[left..mid].to_vec();
    let mut r = a[mid..right].to_vec();
    // 番兵の挿入
    l.push(('A', std::usize::MAX));
    r.push(('A', std::usize::MAX));
    let mut i = 0;
    let mut j = 0;
    for k in left..right {
        if l[i].1 <= r[j].1 {
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

fn merge_sort(a: &mut [(char, usize)], left: usize, right: usize) -> usize {
    let mut count = 0;
    if left + 1 < right {
        let mid = (left + right) / 2;
        count += merge_sort(a, left, mid);
        count += merge_sort(a, mid, right);
        count += merge(a, left, mid, right);
    }
    count
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




