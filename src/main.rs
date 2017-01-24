extern crate gpio_rust_code;

#[allow(dead_code)]
fn main() {
    // Step 1: Setup an mmap
    let mut result = gpio_rust_code::setup();

    // Step 2: setmode
    // #define BOARD        10
    // #define BCM          11
    // Step 3: setup output
    // Step 4: output true or false
    // Step 5: cleanup
    let set_offset = 7;
    let gpio_target = 18;
    // let CLR_OFFSET = 10;

    let offset = set_offset + gpio_target / 32;
    let shift = gpio_target % 32;
    println!("{:?}", offset);
    println!("{:?}", 1 << shift);
}

#[test]
fn it_opens() {
}
