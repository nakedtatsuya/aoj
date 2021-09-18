use std::io::*;
use std::str::FromStr;
use std::collections::HashSet;

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();
    let mut hash_tables: HashSet<String> = HashSet::new();
    for _ in 0..n {
        let o: String = sc.next();

        match o.as_str() {
            "insert" => {
                hash_tables.insert(sc.next());
            },
            "find" => {
                let key: String = sc.next();
                if hash_tables.contains(&key) {
                    println!("yes");
                } else {
                    println!("no");
                }
            },
            _ => {},
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


