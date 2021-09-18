use std::io::*;
use std::str::FromStr;

fn main(){
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();

    let mut arr: Vec<u32> = vec![];
    for _ in 0..n {
        arr.push(sc.next());
    }

    println!("{}", arr.iter().map(|item| item.to_string()).collect::<Vec<_>>().join(" "));

    for i in 1..n {
        let key = arr[i];
        let mut j = i;

        while j >= 1 && arr[j-1] > key {
            arr[j] = arr[j-1];
            j -= 1;
        }
        arr[j] = key;

        println!("{}", arr.iter().map(|item| item.to_string()).collect::<Vec<_>>().join(" "));
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













