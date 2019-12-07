use lalrpop_util::lalrpop_mod;
use std::io::BufRead;

lalrpop_mod!(calc);

fn main() {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        println!("{:?}", calc::ExprParser::new().parse(&line.unwrap()))
    }
}
