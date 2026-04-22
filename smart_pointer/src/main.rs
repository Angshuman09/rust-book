
fn main() {
    let x = 5;
    let y = &x;

    let b = Box::new(69);
    println!("{}", b);
    println!("{}", y);
}
