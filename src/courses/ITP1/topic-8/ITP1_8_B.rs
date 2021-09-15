fn main(){
    loop {
        
        let mut s: String = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let a = s.trim().to_string();
        
        if a == "0" {
            break;
        }
        
        let mut sum = 0;
        for c in a.chars() {
            // c = '1' '2'
            let c_n = c as u32 - 48;
            sum += c_n;
        }
        
        println!("{}", sum);
        
    }
}

