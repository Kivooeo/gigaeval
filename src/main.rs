#![feature(let_chains)]

mod compile;
use crate::compile as comp;

use comp::stage0::Expression;
fn main() {
    let mut x: Expression = Expression::new("2 - 2 * 2");
    let d = x.tokenize();
    dbg!(&d);
    dbg!(&x);
    if let Ok(_) = d {
        dbg!(&x.lookup());
    }
}
