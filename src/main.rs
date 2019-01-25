extern crate gpio_rust_code;

use gpio_rust_code::SetupFailure;

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
    // let set_offset = 7;
    // let gpio_target = 18;
    // let CLR_OFFSET = 10;

    // let offset = set_offset + gpio_target / 32;
    // let shift = gpio_target % 32;
    // println!("{:?}", offset);
    // println!("{:?}", 1 << shift);

    match result {
        Ok(mmap) => {
            println!("Things went well!");
        }
        Err(error) => {
            match error {
                SetupFailure::DEVMEM => { eprintln!("No access to /dev/mem. Try running as root!"); }
                SetupFailure::MALLOC => { eprintln!("No memory"); }
                SetupFailure::MMAP => { eprintln!("Mmap of GPIO registers failed"); }
                SetupFailure::CPUINFO => { eprintln!("Unable to open /proc/cpuinfo"); }
                SetupFailure::NOT_RPI => { eprintln!("Not running on an RPi!"); }
            }
        }
    }
}

#[test]
fn it_opens() {
}
