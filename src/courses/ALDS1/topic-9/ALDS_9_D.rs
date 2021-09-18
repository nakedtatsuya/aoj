use std::str::FromStr;
use std::{io::*};

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();
    let mut A: Vec<usize> = vec![0; n];
    for i in 0..n {
        A[i] = sc.next();
    }

    A.sort();

    for i in 0..n-1 {
        let mut j = i;
        // 根になるまで
        while j != 0 {
            let k = (j-1)/2;
            // 親と交換
            A.swap(j,k);
            // 親のIndex割り当て
            j = k;
        }
        A.swap(0,i+1);
    }


    for i in 0..n {
        print!(" {}", A[i]);
    }
    print!("\n");
}

fn max_heapify(A: &mut Vec<usize>, i: usize) {
    let l = i * 2;
    let r = i * 2 + 1;
    let mut largest = i;
    if l <= A.len() && A[l-1] > A[i-1] {
        largest = l;
    }

    if r <= A.len() && A[r-1] > A[largest-1] {
        largest = r;
    }

    if largest != i {
        A.swap(i-1, largest-1);
        max_heapify(A, largest);
    }
}

fn heap_sort(A: &mut Vec<usize>) {

    for i in (1..=A.len()/2).rev() {
        max_heapify(A, i);
    }


    let mut heap_size = A.len();
    while heap_size >= 1 {
        A.swap(0, heap_size-1);
        heap_size -= 1;
        max_heapify(A, 1);
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


