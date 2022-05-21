use std::usize::MAX;
use std::io::*;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::io::*;

#[derive(Eq, PartialEq, Clone, Copy, PartialOrd, Ord)]
struct Node {
    point: usize,
    cost: usize
}


fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();
    let mut g: Vec<Vec<usize>> = vec![vec![]; n];


    for _ in 0..n {
        let u: usize = sc.next();
        let k: usize = sc.next();

        for _ in 0..k {
            let v: usize = sc.next();
            let c: usize = sc.next();
            g[u][v] = c;
        }
    }

    let mut heap: BinaryHeap<Node> = BinaryHeap::new();


    for i in 0..n {
        let cost = get_route(&g,i);
        println!("{} {}", i, cost);
        // println!("{}", Prim::solve(g));
    }
}

fn get_route(g: &Vec<Vec<usize>>, end: usize) -> usize {
    const INF: usize = 1 << 60;
    let n = g.len();
    let mut d = vec![INF; n];
    let mut used = vec![false; n];
    d[0] = 0;
    loop {
        let mut mincost = INF;
        let mut u: usize = 0;
        for i in 0..n {
            if !used[i] && d[i] < mincost {
                mincost = d[i];
                u = i;
            }
        }
        if mincost == INF {break;}
        used[u] = true;

        for v in 0..n {
            if g[u][v] != MAX {
                d[v] = d[v].min(d[u] + g[u][v]);
            }
        }
    }

    return d[end];
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
