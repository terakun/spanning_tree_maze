pub mod maze;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("{} [rows] [cols]", args[0]);
        return;
    }
    let r = args[1].parse::<usize>().expect("wrong input");
    let c = args[2].parse::<usize>().expect("wrong input");
    let m = maze::generate_random(r, c);
    m.print();
}
