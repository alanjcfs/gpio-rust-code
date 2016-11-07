extern crate gpio_rust_code;

fn main() {
    let mut result = gpio_rust_code::setup();
    let offset = 7 + 18/32;
    let raw = unsafe { result.as_mut_slice() }; // + offset = 1 << 18%32;
    println!("{:?}", raw);
}

#[test]
fn it_opens() {
}
