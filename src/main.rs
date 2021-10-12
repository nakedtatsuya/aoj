use std::io::*;
use std::str::FromStr;
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();
    let k: isize = sc.next();
    let mut b = vec![0; 1 + n as usize];

    for i in 1..n+1 {
        b[i] = b[i - 1];
        let munber: isize = sc.next();
        b[i] += munber - k;
    }

    let mut bit = Bit::new(b.clone());
    let mut l = 0;
    let mut ans = 0;
    for i in 0..n{
        ans += bit.sum(l + 1);
        bit.add(l,1);
    }

    

    let count = bubble_sort(&mut b);

    println!("{:?}", count);


}

struct Bit {
    n:usize,
    bit:Vec<isize>
}
impl Bit {
    fn new(b: Vec<isize>) -> Self {
        let len = b.len();
        Bit {
            bit:b,
            n: len,
        }
    }
    fn add(&mut self,mut id:usize,x:isize){
        id += 1;
        while id <= self.n {
            self.bit[id] += x;
            let i = id as isize;
            id += (i & (-i)) as usize;
        }
    }
    //sum of [0,i)
    fn sum(&mut self, mut id:usize) -> isize {
        let mut s:isize = 0;
        let mut i = id as isize;
        while i > 0 {
            id = i as usize;
            s += self.bit[id];
            i -= i & (-i);
        }
        s
    }
}


fn bubble_sort(a: &mut [isize]) -> usize {
    let mut count = 0;
    for i in 0..a.len() {
        for j in (i+1..a.len()).rev() {
            if a[j] >= a[j-1] {
                a.swap(j, j-1);
                count += 1
            }
        }
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
