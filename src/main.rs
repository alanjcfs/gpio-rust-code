use std::fs::File;
use std::io::*;


const BCM2708_PERI_BASE_DEFAULT: usize = 0x20000000;
const BCM2709_PERI_BASE_DEFAULT: usize = 0x3f000000;
const GPIO_BASE_OFFSET: usize = 0x200000;
const FSEL_OFFSET: usize = 0;
const SET_OFFSET: usize = 7;
const CLR_OFFSET: usize = 10;
const PINLEVEL_OFFSET: usize = 13;
const EVENT_DETECT_OFFSET: usize = 16;
const RISING_ED_OFFSET: usize = 19;
const FALLING_ED_OFFSET: usize = 22;
const HIGH_DETECT_OFFSET: usize = 25;
const LOW_DETECT_OFFSET: usize = 28;
const PULLUPDN_OFFSET: usize = 37;
const PULLUPDNCLK_OFFSET: usize = 38;

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
