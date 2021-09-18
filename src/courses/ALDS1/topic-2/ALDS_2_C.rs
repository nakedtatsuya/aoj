use std::io::*;
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
struct Tranp {
    pub m: String,
    pub n: u32,
    pub i: usize,
}

impl Tranp {
    pub fn new(mut input: String, i: usize) -> Self {
        let (m, n) = {
            let n: u32 = input.pop().unwrap().to_string().parse::<u32>().unwrap();
            let m: String = input;
            (m, n)
        };
        Self {
            m,
            n,
            i,
        }
    }

    pub fn get_full(&self) -> String {
        let n = self.n.to_string();
        let m = self.m.clone();
        m + &n
    }
    pub fn get_vec(&self) -> (char, char, usize) {
        let n = self.n.to_string();
        let m = self.m.clone();
        (m.chars().next().unwrap(), n.chars().next().unwrap(), self.i)
    }
}


fn main(){

    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();

    let mut vec: Vec<Tranp> = Vec::new();
    for i in 0..n {
        let tranp = Tranp::new(sc.next(), i);
        vec.push(tranp);
    }



    let mut buble_arr = vec.clone();
    // buble sort
    for i in 0..n {
        for j in (i+1..n).rev() {
            if buble_arr[j].n < buble_arr[j-1].n {
                let tmp = buble_arr[j].clone();
                buble_arr[j] = buble_arr[j-1].clone();
                buble_arr[j-1] = tmp;
            }
        }
    }
    let res: Vec<String> = buble_arr.iter().map(|card| card.get_full()).collect();
    println!("{}", res.join(" "));
    let res: Vec<(char, char, usize)> = buble_arr.iter().map(|card| card.get_vec()).collect();
    println!("{}", is_stable(&res));



    fn is_stable(c: &Vec<(char, char, usize)>) -> &str {
        let n = c.len();
        for i in 0..n {
            for j in i+1..n {
                if c[i].1 == c[j].1 && c[i].2 > c[j].2 {
                    return "Not stable";
                }
            }
        }
        "Stable"
    }


    let mut select_arr = vec.clone();
    for i in 0..n {
        let mut mini = i;
        for j in i..n {
            if select_arr[j].n < select_arr[mini].n {
                mini = j;
            }
        }
        if mini != i {
            select_arr.swap(i, mini);
        }
    }
    let res: Vec<String> = select_arr.iter().map(|card| card.get_full()).collect();
    println!("{}", res.join(" "));
    let res: Vec<(char, char, usize)> = select_arr.iter().map(|card| card.get_vec()).collect();
    println!("{}", is_stable(&res));
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


