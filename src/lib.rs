extern crate libc;
extern crate memmap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::mem;
use std::result::Result;
use std::io::Error;
use memmap::{Mmap, Protection};

const BCM2708_PERI_BASE_DEFAULT: usize = 0x20000000;
const BCM2709_PERI_BASE_DEFAULT: usize = 0x3f000000;
const GPIO_BASE_OFFSET:          usize = 0x20200000;
const FSEL_OFFSET:               usize = 0;  // 0x0000
const SET_OFFSET:                usize = 7;  // 0x001c / 4
const CLR_OFFSET:                usize = 10; // 0x0028 / 4
const PINLEVEL_OFFSET:           usize = 13; // 0x0034 / 4
const EVENT_DETECT_OFFSET:       usize = 16; // 0x0040 / 4
const RISING_ED_OFFSET:          usize = 19; // 0x004c / 4
const FALLING_ED_OFFSET:         usize = 22; // 0x0058 / 4
const HIGH_DETECT_OFFSET:        usize = 25; // 0x0064 / 4
const LOW_DETECT_OFFSET:         usize = 28; // 0x0070 / 4
const PULLUPDN_OFFSET:           usize = 37; // 0x0094 / 4
const PULLUPDNCLK_OFFSET:        usize = 38; // 0x0098 / 4

const PAGE_SIZE:  usize = 4 * 1024;
const BLOCK_SIZE: usize = 4 * 1024;

pub enum SetupFailure {
    DEVMEM,
    MALLOC,
    MMAP,
    CPUINFO,
    NOT_RPI,
}

const INPUT:              usize = 1;// is really 0 for control register!
const OUTPUT:             usize = 0;// is really 1 for control register!
const ALT0:               usize = 4;

const HIGH:               usize = 1;
const LOW:                usize = 0;

const PUD_OFF:            usize = 0;
const PUD_DOWN:           usize = 1;
const PUD_UP:             usize = 2;

// int setup(void);
pub fn setup() -> Result<memmap::Mmap, SetupFailure> {
    let mut fp = match File::open("/proc/device-tree/soc/ranges") {
        Ok(fp) => fp,
        Err(_) => {
            return Err(SetupFailure::NOT_RPI)
        }
    };
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
    return Ok(mmap);
}

// void setup_gpio(int gpio, int direction, int pud);
// int gpio_function(int gpio);
// void output_gpio(int gpio, int value);
// int input_gpio(int gpio);
// void set_rising_event(int gpio, int enable);
// void set_falling_event(int gpio, int enable);
// void set_high_event(int gpio, int enable);
// void set_low_event(int gpio, int enable);
// int eventdetected(int gpio);
// void cleanup(void);
