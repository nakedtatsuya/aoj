use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin.bytes().map(|c| c.expect("faild") as char).skip_while(|c| c.is_whitespace()).take_while(|c| !c.is_whitespace()).collect();
    token.parse().ok().expect("faild parse")
}
fn main(){
    let w: i32 = read();
    let h: i32 = read();
    let x: i32 = read();
    let y: i32 = read();
    let r: i32 = read();
    if x - r >= 0 && x + r <= w && y - r >=0 && y + r <= h{
        println!("Yes");
    }else{
        println!("No");
    }
}
