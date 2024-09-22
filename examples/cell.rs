use std::cell::Cell;

fn main() {
    let a = Cell::new(1);
    f(&a, &a);
}

fn f(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        println!("hello");
    }
}
