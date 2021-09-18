use std::io::*;
use std::str::FromStr;
use std::collections::VecDeque;

struct Process {
    name: String,
    time: usize,
}

impl Process {
    pub fn new(name: String, time: usize) -> Self {
        Process {
            name,
            time
        }
    }

    pub fn culc_time(&mut self, time: usize) {
        self.time =  self.time - time;
    }

    pub fn is_over(&self, time: usize) -> bool {
        self.time > time
    }

    pub fn need_time(&mut self, total_time: usize) {
        self.time = total_time;
    }
}

fn main(){

    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();
    let q: usize = sc.next();

    let mut vec: VecDeque<Process> = VecDeque::new();
    for _ in 0..n {
        vec.push_back(Process::new(sc.next(), sc.next()));
    }


    let mut total_ms: usize = 0;

    while vec.len() != 0 {
        let mut cur = vec.pop_front().unwrap();
        if !cur.is_over(q) {
            total_ms += cur.time;
            cur.need_time(total_ms);
            println!("{} {}", cur.name, cur.time);
        } else {
            total_ms += q;
            cur.culc_time(q);
            vec.push_back(cur);
        }
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


