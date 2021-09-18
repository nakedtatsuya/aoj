use std::str::FromStr;
use std::{io::*};
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();
    let mut bst = BST::new();

    for _ in 0..n {
        let ord: String = sc.next();


        match ord.as_str() {
            "print" => {
                bst.print();
            }
            c => {
                let key: isize = sc.next();
                match c {
                    "insert" => {
                        let priority: isize = sc.next();
                        bst.insert(key, priority);
                    },
                    "find" => bst.find(&key),
                    "delete" => bst.delete(&key),
                    _ => panic!("command error")
                }
            }
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    key: isize,
    priority: isize,
    left: Link,
    right: Link,
}

impl Node {
    pub fn new(key: isize, priority: isize) -> Self {
        Node {
            key,
            priority,
            left: None,
            right: None,
        }
    }

    fn right_rotate(&mut self) {
        let s = self.clone();
        if self.left.is_some() {
            self.key = s.left.as_ref().unwrap().borrow().key;
            self.priority = s.left.as_ref().unwrap().borrow().priority;
            self.left = s.left.as_ref().unwrap().borrow().left.clone();
            self.right = Some(Rc::new(RefCell::new(Node {
                key: s.key,
                priority: s.priority,
                right: s.right,
                left: s.left.as_ref().unwrap().borrow().right.clone(),
            })))
        }
    }

    fn left_rotate(&mut self) {
        let s = self.clone();
        if self.right.is_some() {
            self.key = s.right.as_ref().unwrap().borrow().key;
            self.priority = s.right.as_ref().unwrap().borrow().priority;
            self.right = s.right.as_ref().unwrap().borrow().right.clone();
            self.left = Some(Rc::new(RefCell::new(Node {
                key: s.key,
                priority: s.priority,
                right: s.right.as_ref().unwrap().borrow().left.clone(),
                left: s.left.clone(),
            })))
        }
    }

    fn pre_delete_rotation(&mut self, key: isize) {
        if key < self.key && self.left.is_some() {
            self.left.as_mut().unwrap().borrow_mut().pre_delete_rotation(key);
        } else if key > self.key && self.right.is_some() {
            self.right.as_mut().unwrap().borrow_mut().pre_delete_rotation(key);
        } else if key == self.key {
            if self.left.is_some() && self.right.is_none() {
                self.right_rotate();
                self.pre_delete_rotation(key);
            } else if self.left.is_none() && self.right.is_some() {
                self.left_rotate();
                self.pre_delete_rotation(key);
            } else if self.left.is_some() && self.right.is_some() {
                if self.left.as_ref().unwrap().borrow().priority > self.right.as_ref().unwrap().borrow().priority {
                    self.right_rotate();
                    self.pre_delete_rotation(key);
                } else {
                    self.left_rotate();
                    self.pre_delete_rotation(key);
                }
            }
        }
    }

    fn delete(&mut self, key: &isize) {
        if *key < self.key && self.left.is_some() {
            if self.left.as_ref().unwrap().borrow().key == *key {
                self.left = None;
            } else {
                self.left.as_mut().unwrap().borrow_mut().delete(key);
            }
        } else if *key > self.key && self.right.is_some() {

            if self.right.as_ref().unwrap().borrow().key == *key {
                self.right = None;
            } else {
                self.right.as_mut().unwrap().borrow_mut().delete(key);
            }
        } 
    }

    fn insert(&mut self, key: isize, priority: isize) {
        if key < self.key {
            if let Some(_) = &self.left {
                self.left.as_mut().unwrap().borrow_mut().insert(key, priority);
            } else {
                self.left = Some(Rc::new(RefCell::new(Node::new(key, priority))));
            }

            if self.priority < priority {
                self.right_rotate();
            }
        } else {

            if let Some(_) = &self.right {
                self.right.as_mut().unwrap().borrow_mut().insert(key, priority);
            } else {
                self.right = Some(Rc::new(RefCell::new(Node::new(key, priority))));
            }

            if self.priority < priority {
                self.left_rotate();
            }
        }
    }

    fn in_parse(&self, res: &mut String) {
        self.left.as_ref().map(|l| l.borrow().in_parse(res));
        *res += &format!(" {}", self.key);
        self.right.as_ref().map(|r| r.borrow().in_parse(res));
    }

    fn pre_parse(&self, res: &mut String) {
        *res += &format!(" {}", self.key);
        self.left.as_ref().map(|l| l.borrow().pre_parse(res));
        self.right.as_ref().map(|r| r.borrow().pre_parse(res));
    }

    fn find(&self, key: &isize) -> bool {
        if *key == self.key {
            return true;
        } else if *key < self.key && self.left.is_some() {
            return self.left.as_ref().unwrap().borrow().find(key);
        } else if *key > self.key && self.right.is_some() {
            return self.right.as_ref().unwrap().borrow().find(key);
        }
        false
    }
    
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct BST {
    root: Link,
}

impl BST {
    pub fn new() -> Self {
        Self { root: None, }
    }

    fn insert(&mut self, key: isize, priority: isize) {
        if let Some(node) = self.root.as_mut() {
            node.borrow_mut().insert(key, priority);
        } else {
            self.root = Some(Rc::new(RefCell::new(Node::new(key, priority))));
        }
    }

    fn find(&self, key: &isize) {
        let mut res = false;
        if let Some(root) = &self.root {
            res = root.borrow().find(key);
        }
        println!("{}", if res {"yes"} else {"no"})
    }

    fn delete(&mut self, key: &isize) {
        self.root.as_mut().map(|root| root.borrow_mut().pre_delete_rotation(*key));
        self.root.as_mut().map(|root| root.borrow_mut().delete(key));
    }

    pub fn in_parse(&self) -> String {
        let mut res = String::new();
        self.root.as_ref().map(|root| root.borrow().in_parse(&mut res));
        res
    }

    fn pre_parse(&self) -> String {
        let mut res = String::new();
        self.root.as_ref().map(|root| root.borrow().pre_parse(&mut res));
        res
    }

    pub fn print(&self) {
        println!("{}\n{}", self.in_parse(), self.pre_parse());
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

