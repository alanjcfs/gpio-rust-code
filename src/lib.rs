extern crate libc;
// extern crate memmap;
use std::fs::File;
use std::io;
// use memmap::{Mmap, Protection};
use libc::mmap;
use std::ptr;
use std::os::unix::io::AsRawFd;

// const BCM2708_PERI_BASE_DEFAULT: usize = 0x20000000;
// const BCM2709_PERI_BASE_DEFAULT: usize = 0x3f000000;
// const GPIO_BASE_OFFSET: usize = 0x200000;
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


const SETUP_OK: usize =           0;
// const SETUP_DEVMEM_FAIL: usize =  1;
// const SETUP_MALLOC_FAIL: usize =  2;
const SETUP_MMAP_FAIL: usize =    3;
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

pub fn setup() /* -> Result<Mmap, io::Error> */ {
    if let Ok(mem_fd) = File::open("/dev/gpiomem") {
        let wut = unsafe {
        // addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, offset: off_tI
            libc::mmap(ptr::null_mut(), 4096usize, libc::PROT_READ|libc::PROT_WRITE, libc::MAP_SHARED, mem_fd.as_raw_fd(), 0)
        };
        println!("{:?}", wut);
        // gpio_map = (uint32_t *)mmap(NULL, BLOCK_SIZE, PROT_READ|PROT_WRITE, MAP_SHARED, mem_fd, 0);
        // Mmap::open(&mem_fd, Protection::ReadWrite)
    } else {
        panic!("Could not query /dev/gpiomem")
    }
}
