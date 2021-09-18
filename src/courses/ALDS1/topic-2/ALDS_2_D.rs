use std::io::*;
use std::str::FromStr;

fn main(){

    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();

    let mut vec: Vec<usize> = Vec::new();
    for _ in 0..n {
        vec.push(sc.next());
    }
    
    fn insertion_sort(A: &mut Vec<usize>, n: usize, g: usize) -> usize {
        let mut count = 0;
        for i in g..n {
            let tmp = A[i];
            let mut j = i;
            while j >= g && A[j-g] > tmp {
                A[j] = A[j-g];
                j = j - g;
                count += 1;
            }
            A[j] = tmp;
        }
        count
    }

    fn shell_sort(A: &mut Vec<usize>, n: usize) {
        let mut count = 0;
        let mut h = 1;
        
        let mut G = vec![];
        while h <= n {
            G.push(h);
            h = 3 * h + 1;
        }
        let m = G.len();
        println!("{}", m);
        G.reverse();
        let s = G.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
        println!("{}", s);

        for i in 0..m {
            count += insertion_sort(A, n, G[i]);
        }
        println!("{}", count);
    }

    shell_sort(&mut vec, n);
    let res: Vec<String> = vec.iter().map(|card| card.to_string()).collect();
    println!("{}", res.join(" "));
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


