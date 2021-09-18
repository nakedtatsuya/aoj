use std::str::FromStr;
use std::{io::*, vec};

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();
    let mut nodes: Vec<Node> = vec![Node::new(); n];
    for i in 0..n {
        let id: usize = sc.next();
        let left: i32 = sc.next();
        let right: i32 = sc.next();
        if right >= 0 {
            nodes[id].right = Some(right as usize);
            nodes[id].degree += 1;
            nodes[right as usize].parent = Some(id);
        }

        if left >= 0 {
            nodes[id].left = Some(left as usize);
            nodes[id].degree += 1;
            nodes[left as usize].parent = Some(id);
        }

        if left >= 0 && right >= 0 {
            nodes[left as usize].sibling = Some(right as usize);
            nodes[right as usize].sibling = Some(left as usize);
        }
    }
    let mut r: usize = 0;
    for i in 0..n {
        if nodes[i].parent.is_none() {
            r = i;
        }
    }

    dfs(&mut nodes, r, 0);
    calc_height_inner(&mut nodes, r);

    println!("Preorder");
    pre_parse(&nodes, Some(r));
    println!("\nInorder");
    in_parse(&nodes, Some(r));
    println!("\nPostorder");
    post_parse(&nodes, Some(r));
    println!();
}

fn pre_parse(nodes: &Vec<Node>, v: Option<usize>) {
    if let Some(u) = v {
        print!(" {}", u);
        pre_parse(nodes, nodes[u].left);
        pre_parse(nodes, nodes[u].right);
    } else {
        return;
    }
}

fn in_parse(nodes: &Vec<Node>, v: Option<usize>) {
    if let Some(u) = v {
        in_parse(nodes, nodes[u].left);
        print!(" {}", u);
        in_parse(nodes, nodes[u].right);
    } else {
        return;
    }
}

fn post_parse(nodes: &Vec<Node>, v: Option<usize>) {
    if let Some(u) = v {
        post_parse(nodes, nodes[u].left);
        post_parse(nodes, nodes[u].right);
        print!(" {}", u);
    } else {
        return;
    }
}

fn dfs(nodes: &mut Vec<Node>, u: usize, d: usize) {
    nodes[u].depth = d;
    if let Some(v) = nodes[u].left {
        dfs(nodes, v, d + 1);
    }
    if let Some(v) = nodes[u].right {
        dfs(nodes, v, d + 1);
    }
}

fn calc_height_inner(nodes: &mut Vec<Node>, u: usize) -> usize {
    if nodes[u].left.is_none() && nodes[u].right.is_none() {
        nodes[u].height = 0;
        return 0;
    }

    let mut ret = 0;
    if let Some(v) = nodes[u].left {
        ret = std::cmp::max(ret, calc_height_inner(nodes, v) + 1);
    }
    if let Some(v) = nodes[u].right {
        ret = std::cmp::max(ret, calc_height_inner(nodes, v) + 1);
    }

    nodes[u].height = ret;
    return ret;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    sibling: Option<usize>,
    depth: usize,
    height: usize,
    degree: usize,
}

impl Node {
    pub fn new() -> Self {
        Node {
            left: None,
            right: None,
            parent: None,
            depth: 0,
            height: 0,
            sibling: None,
            degree: 0,
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

