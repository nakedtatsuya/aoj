fn main(){
        let (r, c) = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_right().to_owned();
            let mut ws = s.split_whitespace();
            let r: usize = ws.next().unwrap().parse().unwrap();
            let c: usize = ws.next().unwrap().parse().unwrap();
            (r, c)
        };
        
        let mut arr: Vec<Vec<usize>> = vec![vec![0; c+1]; r+1];
        
        for i in 0..r {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_right().to_owned();
            let mut ws = s.split_whitespace();
            for j in 0..c {
               arr[i][j] = ws.next().unwrap().parse().unwrap();
               
               if j == c - 1 {
                    let sum = arr[i].iter().sum();
                    arr[i][c] = sum;
               }
               
            }
        }
        
        for i in 0..(c+1) {
            let mut sum = 0;
            for j in 0..r {
               sum += arr[j][i];
               
               if j == r-1 {
                   arr[r][i] = sum;
               }
            }
            
        }

        
        
        // 出力
        
        for i in 0..r+1 {
            for j in 0..c+1 {
                if j != c {
                    print!("{} ", arr[i][j]);
                } else {
                    println!("{}", arr[i][j]);
                }
            }
        }
}
