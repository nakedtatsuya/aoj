use std::io::*;
use std::str::FromStr;

fn main(){
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();

    let mut vec: Vec<u32> = Vec::new();
    for _ in 0..n {
        vec.push(sc.next());
    }
    let mut count = 0;
    for i in 0..n {
        let mut mini = i;
        for j in i..n {
            if vec[j] < vec[mini] {
                mini = j;
            }
        }
        if mini != i {
            vec.swap(i, mini);
            count += 1;
        }
    }
    let res: Vec<String> = vec.iter().map(|i| i.to_string()).collect();
    println!("{}", res.join(" "));
    println!("{}", count);
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


