use std::io::*;
use std::str::FromStr;

enum Direction {
    E,
    S,
    W,
    N
}

struct Dice {
    f: usize,
    b: usize,
    w: usize,
    e: usize,
    s: usize,
    n: usize,
}

impl Dice {
    fn new(num: [usize;6]) -> Dice {
        Dice {
            f: num[0],
            s: num[1],
            e: num[2],
            w: num[3],
            n: num[4],
            b: num[5],
        }
    }

    fn roll(&mut self, d: Direction) -> Dice {
        let (f, s, e, w, n, b) = (self.f, self.s, self.e, self.w, self.n, self.b);
        match d{
            Direction::E => {
                Dice {f: w, b: e, w: b, e: f, s, n}
            },
            Direction::W => {
                Dice {f: e, b: w, w: f, e: b, s, n}
            },
            Direction::S => {
                Dice {f: n, b: s, w, e, s: f, n: b}
            },
            Direction::N => {
                Dice {f: s, b: n, w, e, s: b, n: f}
            },
            _ => panic!()
        }
    }
}

fn main(){
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let mut dice = Dice::new([sc.next(), sc.next(), sc.next(), sc.next(), sc.next(), sc.next()]);
    let orders: String = sc.next();

    for c in orders.chars() {
        match c {
            'E' => {
                dice = dice.roll(Direction::E);
            },
            'W' => {
                dice = dice.roll(Direction::W);
            },
            'S' => {
                dice = dice.roll(Direction::S);
            },
            'N' => {
                dice = dice.roll(Direction::N);
            },
            _ => panic!()
        }
    }

    println!("{}", dice.f);
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


