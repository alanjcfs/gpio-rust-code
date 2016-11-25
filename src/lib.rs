extern crate libc;
extern crate memmap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::mem;
use memmap::{Mmap, Protection};

// const BCM2708_PERI_BASE_DEFAULT: usize = 0x20000000;
// const BCM2709_PERI_BASE_DEFAULT: usize = 0x3f000000;
const GPIO_BASE_OFFSET: usize = 0x20200000;
// const FSEL_OFFSET: usize = 0;
// const SET_OFFSET: usize = 7;
// const CLR_OFFSET: usize = 10;
// const PINLEVEL_OFFSET: usize = 13;
// const EVENT_DETECT_OFFSET: usize = 16;
// const RISING_ED_OFFSET: usize = 19;
// const FALLING_ED_OFFSET: usize = 22;
// const HIGH_DETECT_OFFSET: usize = 25;
// const LOW_DETECT_OFFSET: usize = 28;
// const PULLUPDN_OFFSET: usize = 37;
// const PULLUPDNCLK_OFFSET: usize = 38;


// const SETUP_OK: usize =           0;
// const SETUP_DEVMEM_FAIL: usize =  1;
// const SETUP_MALLOC_FAIL: usize =  2;
// const SETUP_MMAP_FAIL: usize =    3;
// const SETUP_CPUINFO_FAIL: usize = 4;
// const SETUP_NOT_RPI_FAIL: usize = 5;
// const INPUT: usize =  1;// is really 0 for control register!
// const OUTPUT: usize = 0;// is really 1 for control register!
// const ALT0: usize =   4;
// const HIGH: usize = 1;
// const LOW: usize =  0;
// const PUD_OFF: usize =  0;
// const PUD_DOWN: usize = 1;
// const PUD_UP: usize =   2;

pub fn setup() -> memmap::Mmap{
    let mut fp = File::open("/proc/device-tree/soc/ranges").unwrap();
    let mut buf: [u8; 4] = [0, 0 ,0, 0];
    let peribase: usize;

    fp.read(&mut buf[..]).unwrap();
    // peribase = buf[0] << 24 | buf[1] << 16 | buf[2] << 8 | buf[3] << 0 as u32;
    unsafe {
        peribase = mem::transmute::<[u8;4], u32>(buf) as usize;
    }
    // println!("{:?}", peribase);
    let gpio_base = peribase as usize + GPIO_BASE_OFFSET;
    let file =  OpenOptions::new().read(true).write(true).open("/dev/mem").unwrap();
    let mmap = Mmap::open_with_offset(&file, Protection::ReadWrite, GPIO_BASE_OFFSET, 4096 + 4095).unwrap();
    return mmap;
}
