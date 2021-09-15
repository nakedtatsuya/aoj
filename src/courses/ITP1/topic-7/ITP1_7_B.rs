fn main(){
    loop {
        let (n, x) = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_right().to_owned();
            let mut ws = s.split_whitespace();
            let n: usize = ws.next().unwrap().parse().unwrap();
            let x: usize = ws.next().unwrap().parse().unwrap();
            (n, x)
        };
        
        if n == 0 && x == 0 {
            break;
        }
        
        let mut result = 0;

        for i in 1..(n+1) {
           for j in 1..i {
               for k in 1..j {
                   if i+j+k == x {
                        result += 1;
                   }
                }
            }
        }
        
        println!("{}", result);
        
    }
}
