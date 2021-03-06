fn main(){

    macro_rules! input {
        (source = $s:expr, $($r:tt)*) => {
            let mut iter = $s.split_whitespace();
            input_inner!{iter, $($r)*}
        };
        ($($r:tt)*) => {
            let mut s = {
                use std::io::Read;
                let mut s = String::new();
                std::io::stdin().read_to_string(&mut s).unwrap();
                s
            };
            let mut iter = s.split_whitespace();
            input_inner!{iter, $($r)*}
        };
    }
    
    macro_rules! input_inner {
        ($iter:expr) => {};
        ($iter:expr, ) => {};
    
        ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
            let $var = read_value!($iter, $t);
            input_inner!{$iter $($r)*}
        };
    }
    
    macro_rules! read_value {
        ($iter:expr, ( $($t:tt),* )) => {
            ( $(read_value!($iter, $t)),* )
        };
    
        ($iter:expr, [ $t:tt ; $len:expr ]) => {
            (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
        };
    
        ($iter:expr, chars) => {
            read_value!($iter, String).chars().collect::<Vec<char>>()
        };
    
        ($iter:expr, usize1) => {
            read_value!($iter, usize) - 1
        };
    
        ($iter:expr, $t:ty) => {
            $iter.next().unwrap().parse::<$t>().expect("Parse error")
        };
    }



    input!{
        n: usize,
        v: [i64; n],
    }
    let min: &i64 = v.iter().min().unwrap();
    let max: &i64 = v.iter().max().unwrap();
    let sum: i64 = v.iter().sum();
    println!("{} {} {}", min, max, sum);
    

}
