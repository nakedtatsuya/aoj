fn main(){
        let (s) = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_right().to_owned();
            (s)
        };
        
        let arr: Vec<char> = s.chars().map(|c| {
            if c.is_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c.to_ascii_uppercase()
            }
        }).collect();

        // 出力 n x l
        for i in 0..arr.len() {
            print!("{}", arr[i]);
        }

}

