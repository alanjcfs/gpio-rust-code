use std::fs::File;
use std::io::*;

fn main() {
  let f = open_file("/dev/gpiomem");
  let mut buf = [0; 4];
  println!("{:?}", f);

  let mut g = open_file("/proc/device-tree/soc/ranges").unwrap();


  g.seek(SeekFrom::Start(4));

  g.read(&mut buf[..]);

  println!("{:?}", buf);
}

fn open_file(dev: &str) -> std::result::Result<File, Error> {
  File::open(dev)
}

/* Becvause escalated privileges are needed to access /dev/gpiomem, we need to mock the device */
#[test]
fn it_opens() {
  assert_eq!(open_file("/dev/gpiomem").is_ok(), false);
}
