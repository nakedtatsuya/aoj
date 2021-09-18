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
    let mut B: Vec<usize> = vec![0; n];
    let mut V: Vec<bool> = vec![false; n];
    for i in 0..n {
        B[i] = A[i];
    }

    B.sort();
    for i in 0..n {
        if B[i] == A[i] {
            V[i] == true;
        }
    }
    let mut T: Vec<usize> = vec![0; 10001];
    for i in 0..n {
        T[B[i]] = i;
    }

    let mut ans = 0;
    let min = *A.iter().min().unwrap();

    for i in 0..n {
        
        let mut cur = i;
        let mut S = 0;
        let mut m = 10000;
        let mut an: isize = 0;
        loop {
            V[cur] = true;
            an += 1;
            let v = A[cur];
            m = m.min(v);
            S += v;
            cur = T[v];
            if V[cur] {
                break;
            }
        }
        ans += (S as isize + (an - 2) * m as isize).min(m as isize + S as isize +(an+1)*min as isize) as isize;
    }

    println!("{}", ans);



    // let r = A.len() - 1;
    // let cost = quick_sort(&mut A, 0, r);
    // println!("{}", cost);

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

fn min_cost_sort() {

}

fn quick_sort(A: &mut Vec<usize>, p: usize, r: usize) -> usize {
    let mut cost: usize = 0;
    if p < r {
        let (q, mut c) = partition(A, p, r);
        c += quick_sort(A, p, q-1);
        c += quick_sort(A, q+1, r);
        cost += c;
    }
    cost
}

fn partition(A: &mut Vec<usize>, p: usize, r: usize) -> (usize, usize) {
    let x = A[r];
    let mut i = p;
    let mut cost: usize = 0;
    for j in p..r {
        if A[j] <= x {
            A.swap(i, j);
            cost += A[i]+A[j];
            i += 1;
        }
    }
    A.swap(i, r);
    cost += A[i]+A[r];
    (i, cost)
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




