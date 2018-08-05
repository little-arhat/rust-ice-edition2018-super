use rand::random;

mod ttt;
use crate::ttt::S;

fn main() {
    let x: u8 = random();
    let mut b = ttt::it::N::new();
    println!("Hello, world!: {}/{}", x, b.s());
}
