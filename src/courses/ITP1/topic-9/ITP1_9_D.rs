use std::io::*;
use std::str::FromStr;
fn main(){
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    let mut text: String = sc.next();
    let n: u32 = sc.next();
    

    for _ in 0..n {
        let s: String = sc.next();
        let a: usize = sc.next();
        let b: usize = sc.next::<usize>() + 1;

        match s.as_ref() {
            "print" => {
                println!("{}", &text[a..b]);
            },
            "reverse" => {
                let x = &text[..a];
                let y = &text[a..b].chars().rev().collect::<String>();
                let z = &text[b..];
                text = format!("{}{}{}", x, y, z);
            },
            "replace" => {
                let replace_text: String = sc.next();
                let x = &text[..a];
                let y = &text[b..];
                text = format!("{}{}{}", x, replace_text, y);
            },
            _ => panic!(),
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












