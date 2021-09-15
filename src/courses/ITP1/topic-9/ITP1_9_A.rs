use std::io::*;
use std::str::FromStr;
fn main(){
    let cin = stdin();
    let mut sc = Scanner::new(cin.lock());

    let W = sc.next::<String>();
    
    let mut T: Vec<String> = vec![];
    
    loop {
        let mut text_line = sc.next::<String>();
        if &text_line == "END_OF_TEXT" {
            break;
        }
        
        T.push(text_line.to_ascii_lowercase());
    }
    
    let a: Vec<&String> = T.iter().filter(|&t| t == &W).collect();
    
    println!("{}", a.len());
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

    fn chars(&mut self) -> Vec<char> {
        self.next::<String>().chars().collect()
    }

    fn char(&mut self) -> char {
        self.chars()[0]
    }
}
















