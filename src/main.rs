use std::fs::File;
use std::io::*;

extern crate gpio_rust_code;

fn main() {
  let mut f = open_file("/dev/gpiomem").unwrap();
  let mut buf = Vec::new();

  let result = f.read_to_end(&mut buf);

  println!("{:?}", result);
}

fn open_file(dev: &str) -> std::result::Result<File, Error> {
  File::open(dev)
}

/* Because escalated privileges are needed to access /dev/gpiomem, we need to
 * mock the device */
#[test]
fn it_opens() {
  assert_eq!(open_file("/dev/gpiomem").is_ok(), false);
}
