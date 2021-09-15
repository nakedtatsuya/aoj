fn main(){
    let (n, m) = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_right().to_owned();
        let mut ws = s.split_whitespace();
        let n: usize = ws.next().unwrap().parse().unwrap();
        let m: usize = ws.next().unwrap().parse().unwrap();
        (n, m)
    };
    
    let mut a = vec![vec![0; m]; n];
    
    for i in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_right().to_owned();
        let mut ws = s.split_whitespace();
        for j in 0..m {
            a[i][j] = ws.next().unwrap().parse().unwrap();
        }
    }
    
    let mut b = vec![0; m];
    
    for x in 0..m {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_right().to_owned();
        let mut ws = s.split_whitespace();
        b[x] = ws.next().unwrap().parse().unwrap();
    }
    
    for i in 0..n {
        let mut res = 0;
        for j in 0..m {
            res += a[i][j] * b[j];
        }
        println!("{}", res);
    }
}


