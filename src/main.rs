use std::env;
use std::fs::File;
use std::io::Read;

mod lexer;
mod parser;

use parser::*;

fn eval(ast: &Node) -> i32 {
    match ast {
        Node::Node { v, children } => {
            let args: Vec<i32> = children.iter().map(|i| eval(i)).collect();
            v.apply(&args)
        }
        Node::Leaf(LeafVal::Int(v)) => {
            *v
        }
        Node::Leaf(LeafVal::Sym(_)) => panic!("Cannot eval symbol"),
    }
}

fn main() {
    let mut args = env::args();

    if args.len() != 2 {
        panic!(
            "Exactly one argument is expected, {} were supplied.",
            args.len() - 1
        );
    }

    let path = args.nth(1).unwrap();
    let mut file = File::open(path).unwrap();
    let metadata = file.metadata().unwrap();
    let mut s = Vec::<u8>::with_capacity(metadata.len() as usize);

    file.read_to_end(&mut s).unwrap();

    let ast = parser::expr(&s);

    println!("Evaluating {ast}: {}", eval(&ast));
}
