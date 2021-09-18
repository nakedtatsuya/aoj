use std::io::*;
use std::str::FromStr;
use std::collections::VecDeque;
use std::cmp::Ordering;

#[derive(Clone, Debug)]
struct Mountain {
    diagram: VecDeque<Point>,
    flood_area: VecDeque<VecDeque<Point>>,
}

impl Mountain {
    pub fn new() -> Self {
        Mountain {
            diagram: Default::default(),
            flood_area: Default::default(),
        }
    }

    pub fn get_mut_diagram(&mut self) -> &mut VecDeque<Point> {
        &mut self.diagram
    }

    pub fn check_exsist_same_y(&mut self, c_i: usize) -> Option<VecDeque<Point>> {
        let mut res: VecDeque<Point> = VecDeque::new();
        for o_i in c_i+1..self.diagram.len() {
            
            if self.diagram[c_i-1].y == self.diagram[o_i].y {
                for x in c_i-1..o_i+1 {
                    res.push_back(self.diagram[x]);
                }
                // println!("{:?} {:?}", self.diagram[c_i-1], self.diagram[o_i]);
                return Some(res)
            }
        }
        None
    }

    pub fn test(&mut self) {
        let mut last_index = 1;

        while last_index != self.diagram.len()-1 {
            for i in last_index..self.diagram.len() {
                let pre = self.diagram[i-1];
                let cur = self.diagram[i];
                if pre.y > cur.y {
                    if let Some(flood_area) = self.check_exsist_same_y(i) {
                        last_index = flood_area.back().unwrap().x as usize;
                        self.flood_area.push_back(flood_area);
                        break;
                    } else {
                        continue;
                    }
                } else {
                    last_index +=1;
                    break;
                }
            }
        }
    }

    pub fn caluc_flood(&self) -> (usize, VecDeque<usize>) {

        let mut res = 0.0;
        let mut tmp: VecDeque<usize> = VecDeque::new();
        for i in 0..self.flood_area.len() {

            for x in 1..self.flood_area[i].len() {

                let pre = self.flood_area[i][x-1];
                let cur = self.flood_area[i][x];

                match cur.y.cmp(&pre.y) {
                    Ordering::Less => {
                        res += self.flood_area[i][0].y as f32 - cur.y as f32 - 0.5;
                    },
                    Ordering::Equal => {
                        res += self.flood_area[i][0].y as f32 - cur.y as f32;
                    },
                    Ordering::Greater => {
                        res += self.flood_area[i][0].y as f32 - cur.y as f32 + 0.5;
                    },
                }


            }
            tmp.push_back(res as usize);
            res = 0.0;

        }
        (tmp.iter().sum(), tmp)
    }
}

#[derive(Clone, Debug, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Point {
            x,
            y,
        }
    }
}
fn adjust<T, U>(ar: &mut Vec<(T, U)>)
  where T: Sized + PartialOrd,
        U: Sized + std::ops::Add<Output = U>
{
  let len = ar.len();
  if len < 2 { return; }
  if ar[len - 1].0 > ar[len - 2].0 { return; }
  let last = ar.pop().unwrap();
  let b_last = ar.pop().unwrap();
  ar.push((last.0, last.1 + b_last.1));
  adjust(ar);
}
fn main(){

    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: String = sc.next();

    let mut mountain = Mountain::new();
    let mut y = 0;
    mountain.get_mut_diagram().push_back(Point::new(0, y));
    let mut test = vec![];
    let mut stack = vec![];
    let mut ar: Vec<(usize, usize)> = vec![];

    for (i, c) in n.chars().enumerate() {
        let x = i as isize + 1;
        match c {
            '/' => {
                y += 1;
                mountain.get_mut_diagram().push_back(Point::new(x, y));
                test.push('/');
                if let Some(left_index) = stack.pop() {
                    let diff = i - left_index;
                    ar.push((left_index, diff));
                    adjust(&mut ar);
                }
            },
            '\\' => {
                stack.push(i);
                y -= 1;
                mountain.get_mut_diagram().push_back(Point::new(x, y));
                test.push('\\');

                

                

            },
            '_' => {
                mountain.get_mut_diagram().push_back(Point::new(x, y));
                test.push('_');
            },
            _ => {},
        }
    }
    // if test.contains(&'/') && test.contains(&'\\') {
    //     mountain.test();
    // }
    // let (res, arr) = mountain.caluc_flood();
    // let p: Vec<String> = arr.iter().map(|x| x.to_string()).collect();
    // println!("{}", res);
    // if arr.len() == 0 {
    //     println!("{}", arr.len());
    // } else {
    //     println!("{} {}", arr.len(), p.join(" "));
    // }
  
    println!("{}", ar.iter().map(|&(_, u)| u).sum::<usize>());
    print!("{}", ar.len());

    for &(_, u) in ar.iter() {
        print!(" {}", u);
      }
    println!();

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


