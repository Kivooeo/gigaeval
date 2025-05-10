#![feature(let_chains)]
#![feature(if_let_guard)]
mod compile;
use crate::compile as comp;
use comp::stage0::Expression;
use comp::stage0::{Operand, Operator};
use comp::stage1::Stack;
use compile::stage0::Token;

fn main() {
    let mut x: Expression = Expression::new("x - y | 2 1");
    let d = x.tokenize();
    dbg!(&d);
    // dbg!(&x);
    let x = Stack::from(&mut x);
    x.print();
}
