use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("% fib N");
    }
    
    let num = args[1].parse().expect("couldn't parse number");

    let mut a = 0;
    let mut b = 1;
    for _ in 0..num {
        b = a + b;
        a = b - a;
    }

    println!("{}", a);
}
