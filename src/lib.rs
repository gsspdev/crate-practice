pub mod add;

pub fn add_twice(x, y) {
    add(x, y) + add(x, y)
}