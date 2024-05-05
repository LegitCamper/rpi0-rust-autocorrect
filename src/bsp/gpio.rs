use core::{
    arch::asm,
    ptr::{read_volatile, write_volatile},
};

const GPIO_BASE: u32 = 0x2020_0000 as u32;

const GPIO_FSEL0: u32 = GPIO_BASE;
const GPIO_FSEL1: u32 = GPIO_BASE + 0x04;
const GPIO_FSEL2: u32 = GPIO_BASE + 0x08;
const GPIO_FSEL3: u32 = GPIO_BASE + 0x0C;
const GPIO_FSEL4: u32 = GPIO_BASE + 0x10;
const GPIO_FSEL5: u32 = GPIO_BASE + 0x14;

const GPIO_SET0: u32 = GPIO_BASE + 0x1c;
const GPIO_SET1: u32 = GPIO_BASE + 0x20;

const GPIO_CLR0: u32 = GPIO_BASE + 0x28;
const GPIO_CLR1: u32 = GPIO_BASE + 0x2C;

const ACT_PIN: u32 = 47;

pub enum BlinkSpeed {
    FastBlink = 250000,
    SlowBlink = 500000,
}

pub struct GPIO;

impl GPIO {
    pub fn set_output(pin: u32) {
        let reg = pin / 10;
        let register = match reg {
            0 => GPIO_FSEL0,
            1 => GPIO_FSEL1,
            2 => GPIO_FSEL2,
            3 => GPIO_FSEL3,
            4 => GPIO_FSEL4,
            5 => GPIO_FSEL5,
            _ => panic!("You tried to use a pin that is invalid"),
        };

        #[allow(unused)]
        let mut val: u32 = 0;
        unsafe {
            val = read_volatile(register as *mut u32);
        }

        let mut mask: u32 = 0b111;
        let pinnum = pin % 10;
        mask = mask << pinnum * 3;
        val = val & !(mask);
        val |= 1 << pinnum * 3;

        unsafe {
            write_volatile(register as *mut u32, val);
        }
    }

    pub fn set(pin: u32) {
        let bitpos = pin;

        #[allow(unused)]
        let mut val: u32 = 0;
        if pin <= 31 {
            unsafe {
                val = read_volatile(GPIO_SET0 as *mut u32);
            }
            val |= 1 << bitpos;
            unsafe { write_volatile(GPIO_SET0 as *mut u32, val) }
        } else if pin > 31 && pin <= 53 {
            unsafe {
                val = read_volatile(GPIO_SET1 as *mut u32);
            }
            val |= 1 << bitpos;
            unsafe { write_volatile(GPIO_SET1 as *mut u32, val) }
        } else {
            panic!("Pin does not exist");
        }
    }

    pub fn clear(pin: u32) {
        let bitpos = pin;

        #[allow(unused)]
        let mut val: u32 = 0;
        if pin <= 31 {
            unsafe {
                val = read_volatile(GPIO_CLR0 as *mut u32);
            }
            val |= 1 << bitpos;

            unsafe { write_volatile(GPIO_CLR0 as *mut u32, val) }
        } else if pin > 31 && pin <= 53 {
            unsafe {
                val = read_volatile(GPIO_CLR1 as *mut u32);
            }
            val |= 1 << bitpos;

            unsafe { write_volatile(GPIO_CLR1 as *mut u32, val) }
        } else {
            panic!("Pin does not exist");
        }
    }

    pub fn act_blink(speed: BlinkSpeed, n: u32) {
        let speed = speed as u32;

        GPIO::set_output(ACT_PIN);

        for _ in 0..n {
            // Turn act light on
            GPIO::set(ACT_PIN);

            for _ in 1..speed {
                unsafe { asm!("nop") }
            }
            // Turn act light off
            GPIO::clear(ACT_PIN);

            for _ in 1..speed {
                unsafe { asm!("nop") }
            }
        }
    }
}
