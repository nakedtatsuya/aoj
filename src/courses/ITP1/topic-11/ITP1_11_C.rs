use std::io::*;
use std::str::FromStr;
use std::iter::*;
use std::collections::BTreeMap;

enum Roll {
    Forward(Direction),
    Horizontal(Direction),
    Vertical(Direction),
}
enum Direction {
    Left,
    Right,
}

#[derive(Eq, PartialOrd, PartialEq)]
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

    fn roll(self, d: Roll) -> Dice {
        let (f, s, e, w, n, b) = (self.f, self.s, self.e, self.w, self.n, self.b);
        match d{
            Roll::Vertical(Direction::Right) => {
                Dice {f: w, b: e, w: b, e: f, ..self}
            },
            Roll::Vertical(Direction::Left) => {
                Dice {f: e, b: w, w: f, e: b, ..self}
            },
            Roll::Forward(Direction::Right) => {
                Dice {f: n, b: s, s: f, n: b, ..self}
            },
            Roll::Forward(Direction::Left) => {
                Dice {f: s, b: n, s: b, n: f, ..self}
            },
            Roll::Horizontal(Direction::Left) => {
                Dice {e: s, s: w, w: n, n: e, ..self}
            },
            Roll::Horizontal(Direction::Right) => {
                Dice {s: e, e: n, n: w, w: s, ..self}
            },
            _ => panic!()
        }
    }
}

fn main(){
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let mut dice1 = Dice::new([sc.next(), sc.next(), sc.next(), sc.next(), sc.next(), sc.next()]);
    let dice2 = Dice::new([sc.next(), sc.next(), sc.next(), sc.next(), sc.next(), sc.next()]);

    let mut isEqual = false;

    // 上面と前面の組み合わせに対しての右面の値を登録
    'outer: for k in 0..6 {
        // 一回転
        for _ in 0..4 {
            dice1 = dice1.roll(Roll::Horizontal(Direction::Right));
            if dice1 == dice2 {
                isEqual = true;
                break 'outer;
            }
        }

        // 組み合わせ変更
        if k % 2 == 0 {
            dice1 = dice1.roll(Roll::Forward(Direction::Right));
        } else {
            dice1 = dice1.roll(Roll::Vertical(Direction::Right));
        }
    }

    println!("{}", if isEqual {"Yes"} else {"No"});

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













