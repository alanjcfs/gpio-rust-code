extern crate gpio_rust_code;

fn main() {
    let foo = gpio_rust_code::setup();
    println!("{:?}", foo)
}

#[test]
fn it_opens() {
}
