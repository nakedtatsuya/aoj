use std::str::FromStr;
use std::{io::*, vec};

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();

    let mut A: Vec<usize> = vec![0; n];
    let mut nodes: Vec<Node> = vec![Node::new(std::u32::MAX, std::i32::MIN, vec![]); 100000];
    for i in 0..n {
        A[i] = sc.next();
        let k: usize = sc.next();
        let mut c: Vec<usize> = vec![0; k];
        for j in 0..k {
            c[j] = sc.next();
            nodes[c[j]].parent = A[i] as i32;
        }
        nodes[A[i]].id = A[i] as u32;
        nodes[A[i]].child = c;
    }

    for i in 0..n {
        if nodes[A[i]].depth != std::i32::MIN {
            continue;
        }

        let mut d = 0;
        let mut p = nodes[A[i]].parent;
        while p != std::i32::MIN {
            p = nodes[p as usize].parent;
            d += 1;
        }
        nodes[A[i]].depth = d;
    }



    nodes.sort();

    for i in 0..n {
        let node = &nodes[i];
        print_node(node);
    }
}

fn print_node(node: &Node) {
    let t = if node.parent == std::i32::MIN {
            "root"
        } else if !node.child.is_empty() {
            "internal node"
        } else {
            "leaf"
        };

        let p = if node.parent == std::i32::MIN {
            -1
        } else {
            node.parent
        };
        
        println!(
            "node {}: parent = {}, depth = {}, {}, {:?}",
            node.id, p, node.depth, t, node.child
        );
}

fn setDepth(u: usize, p: usize) {

}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum NodeType {
    Root,
    InternalNode,
    Leaf,
    Undifined,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    id: u32,
    parent: i32,
    node_type: NodeType,
    child: Vec<usize>,
    depth: i32,
}

impl Node {
    pub fn new(id: u32, parent: i32, child: Vec<usize>) -> Self {
        Node {
            id,
            child,
            parent,
            node_type: NodeType::Undifined,
            depth: std::i32::MIN,
        }
    }

    // pub fn get_depth(&mut self, nodes: &Vec<Node>) {
    //     let mut d = 0;
    //     let mut p = self.parent;
    //     while p != -1 {
    //         let p_node = nodes.iter().find(|node| node.id == p as usize).unwrap();
    //         p = p_node.parent;
    //         d += 1;
    //     }
    //     self.depth = d;
    // }
}

fn quick_sort(A: &mut Vec<usize>, p: usize, r: usize) -> usize {
    let mut cost: usize = 0;
    if p < r {
        let (q, mut c) = partition(A, p, r);
        c += quick_sort(A, p, q - 1);
        c += quick_sort(A, q + 1, r);
        cost += c;
    }
    cost
}

fn partition(A: &mut Vec<usize>, p: usize, r: usize) -> (usize, usize) {
    let x = A[r];
    let mut i = p;
    let mut cost: usize = 0;
    for j in p..r {
        if A[j] <= x {
            A.swap(i, j);
            cost += A[i] + A[j];
            i += 1;
        }
    }
    A.swap(i, r);
    cost += A[i] + A[r];
    (i, cost)
}

fn merge(a: &mut [(char, usize)], left: usize, mid: usize, right: usize) -> usize {
    let mut count = 0;
    let mut l = a[left..mid].to_vec();
    let mut r = a[mid..right].to_vec();
    // 番兵の挿入
    l.push(('A', std::usize::MAX));
    r.push(('A', std::usize::MAX));
    let mut i = 0;
    let mut j = 0;
    for k in left..right {
        if l[i].1 <= r[j].1 {
            a[k] = l[i];
            i += 1;
        } else {
            a[k] = r[j];
            j += 1;
            count += l.len() - 1 - i;
        }
    }
    count
}

fn merge_sort(a: &mut [(char, usize)], left: usize, right: usize) -> usize {
    let mut count = 0;
    if left + 1 < right {
        let mid = (left + right) / 2;
        count += merge_sort(a, left, mid);
        count += merge_sort(a, mid, right);
        count += merge(a, left, mid, right);
    }
    count
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

