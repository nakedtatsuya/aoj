use std::str::FromStr;
use std::{io::*};

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();
    let mut t = BST::new();

    for i in 0..n {
        let ord: String = sc.next();

        if ord.eq("insert") {
            let val: isize = sc.next();
            t.insert(val);
        } else if ord.eq("print") {
            t.print_inorder();
            print!("\n");
            t.print_preorder();
            print!("\n");
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    key: isize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(key: isize) -> Self {
        Node {
            key,
            left: None,
            right: None,
        }
    }
}

struct BST {
    root: Option<Box<Node>>,
}

impl BST {
    pub fn new() -> Self {
        Self { root: None, }
    }
    pub fn insert(&mut self, val: isize) {
        let z = Box::new(Node::new(val));
        let mut x = &mut self.root;

        while let Some(v) = x {
            if z.key < v.key {
                x = &mut v.left;
            }
            else {
                x = &mut v.right;
            }
        }

        *x = Some(z);
    }

    fn print_preorder_inner(u: &Option<Box<Node>>) {
        if let Some(ref v) = u {
            print!(" {}", v.key);
            Self::print_preorder_inner(&v.left);
            Self::print_preorder_inner(&v.right);
        }
    }
    pub fn print_preorder(&self) {
        Self::print_preorder_inner(&self.root);
    }

    fn print_inorder_inner(u: &Option<Box<Node>>) {
        if let Some(ref v) = u {
            Self::print_inorder_inner(&v.left);
            print!(" {}", v.key);
            Self::print_inorder_inner(&v.right);
        }
    }
    pub fn print_inorder(&self) {
        Self::print_inorder_inner(&self.root);
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

