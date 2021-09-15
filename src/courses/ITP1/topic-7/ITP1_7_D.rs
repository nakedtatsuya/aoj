fn main(){
        let (n, m, l) = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_right().to_owned();
            let mut ws = s.split_whitespace();
            let n: usize = ws.next().unwrap().parse().unwrap();
            let m: usize = ws.next().unwrap().parse().unwrap();
            let l: usize = ws.next().unwrap().parse().unwrap();
            (n, m, l)
        };
        
        // A
        let mut A: Vec<Vec<usize>> = vec![vec![0; m]; n];
        for i in 0..n {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_right().to_owned();
            let mut ws = s.split_whitespace();
            for j in 0..m {
               A[i][j] = ws.next().unwrap().parse().unwrap();
            }
        }
        
        // B
        let mut B: Vec<Vec<usize>> = vec![vec![0; l]; m];
        for i in 0..m {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_right().to_owned();
            let mut ws = s.split_whitespace();
            for j in 0..l {
               B[i][j] = ws.next().unwrap().parse().unwrap();
            }
            
        }

        // 出力 n x l
        for i in 0..n {
            for j in 0..l {
                let mut result = 0;
                // 掛け算
                for x in 0..m {
                    result += A[i][x] * B[x][j]
                }
                print!("{}{}", result, if j == l-1 {"\n"} else {" "});
            }
        }
}
