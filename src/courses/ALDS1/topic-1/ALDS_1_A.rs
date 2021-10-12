use std::io::*;
use std::str::FromStr;

fn main(){
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();
    let p: Vec<f64> = (0..n).map(|_| sc.next()).collect();
    let q: Vec<f64> = (0..=n).map(|_| sc.next()).collect();
    let mut x = vec![0 as f64; p.len() + q.len()];
    let mut mem = vec![vec![None; 2*n+2]; 2*n+2];
    let mut arr: Vec<u32> = vec![];
    for i in 0..x.len() {
        if i % 2 == 0 {
            x[i] = q[i / 2];
        }
        else {
            x[i] = p[i / 2];
        }
    }

    println!("{}", dfs(0, x.len(), 1, &x, &mut mem));

}

fn dfs(l: usize, r: usize, d: i32, p: &Vec<f64>, mem: &mut Vec<Vec<Option<f64>>>) -> f64 {
    if r - l == 1 {
        return p[l];
    }
    else if r <= l {
        return 0 as f64;
    }
    else if let Some(ret) = mem[l][r] {
        return ret;
    }

    let mut ret = 5000.0;
    for mid in l..r {
        if mid % 2 == 0 {
            continue;
        }
        let a = dfs(l, mid, d + 1, p, mem) + dfs(mid + 1, r, d + 1, p, mem);
        if ret > a {
            ret = a;
        }
    }
    for i in l..r {
        ret += p[i];
    }

    mem[l][r] = Some(ret);
    return ret;
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
