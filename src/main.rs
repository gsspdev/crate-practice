use crate::add:add; // add.rs

fn main() {
    let a = 2;
    let b = 3;
    let c = add(a, b); 
    let d = add_twice(a, b);
    println!("{} + {} = {}, {}", a, b, c, d);
}
