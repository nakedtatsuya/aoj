use std::io::Read;

fn main(){
        let mut s: String = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s.make_ascii_lowercase();

        for i in "abcdefghijklmnopqrstuvwxyz".chars() {
            // マッチするアルファベットでFlter。残ったVecのlen()で判定
            let result: Vec<char> = s.chars().filter(|c| c == &i).collect();
            println!("{} : {}", i, result.len());
        }
}