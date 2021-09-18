use std::{io::*, vec};
use std::str::FromStr;

// √3/2a a = 1辺
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: isize = sc.next();
    let start = Point(0.0, 0.0);
    let end = Point(100.0, 0.0);
    start.print();
    koch(n, &start, &end);
    end.print();
}

fn print_vec<T: std::fmt::Display>(v: Vec<T>) {
    for a in &v[0..v.len() - 1] {
        print!("{} ", a);
    }
    println!("{}", v[v.len() - 1]);
}
#[derive(Debug)]
struct Point(f64, f64);

impl Point {
    fn distance(&self, p: &Point) -> f64 {
        let d: f64 = ((self.0 - p.0) * (self.0 - p.0)).sqrt() + ((self.1 - p.1) * (self.1 - p.1)).sqrt();
        d
    }

    fn mid_point(&self, p: &Point) -> Point {
        Point((self.0 + p.0)/2_f64, (self.1 + p.1)/2_f64)
    }

    fn print(&self) {
        println!("{} {}", self.0, self.1);
    }
}

fn get_height(a: f64) -> f64 {
    3_f64.sqrt()/2_f64*a
}

fn koch(n: isize, a: &Point, b: &Point) {
    if n == 0 { return; }

    let th = std::f64::consts::PI * 60.0 / 180.0;

    let s = Point((2.0*a.0 + b.0)/3.0, (2.0*a.1 + b.1)/3.0);
    let t = Point((a.0 + 2.0*b.0)/3.0, (a.1 + 2.0*b.1)/3.0);
    let u = Point((t.0 - s.0)*th.cos() - (t.1 - s.1)*th.sin() + s.0, (t.0 - s.0)*th.sin() + (t.1 - s.1)*th.cos() + s.1);
    koch(n - 1, a, &s);
    println!("{} {}", s.0, s.1);
    koch(n - 1, &s, &u);
    println!("{} {}", u.0, u.1);
    koch(n - 1, &u, &t);
    println!("{} {}", t.0, t.1);
    koch(n - 1, &t, b);
}
// 30, 90, 150, 210, 270, 
// 38.88888889 9.62250449
// 33.33333333 19.24500897
// 44.44444444 19.24500897
enum C {
 Y,


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



