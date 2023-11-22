#![no_std]
#![no_main]

//use core::arch::asm;
use core::panic::PanicInfo;

const FSEL0: u32 = 0x3f20_0000;
const FSEL1: u32 = 0x3f20_0004;
const FSEL2: u32 = 0x3f20_0008;
const GPIO_SET0: u32 = 0x3f20_001c;
const GPIO_CLEAR0: u32 = 0x3f20_0028;
const GPIO_PIN_LEVEL0: u32 = 0x3f20_0034;

struct GPIO;

impl GPIO {
    pub fn set_pin_to_output(pin: u32) {
        let fsel = pin / 10;
        let register = match fsel {
            0 => FSEL0,
            1 => FSEL1,
            2 => FSEL2,
            _ => panic!("ERROR: Couldn't match to a function select register"),
        };

        let mut val: u32;

        unsafe {
            val = core::ptr::read_volatile(register as *mut u32);
        }

        let bit_position = (pin % 10) * 3;
        val &= !(0b111 << bit_position);
        val |= 0b001 << bit_position;

        unsafe {
            core::ptr::write_volatile(register as *mut u32, val);
        }
    }

    pub fn set_pin_to_input(pin: u32) {
        let fsel = pin / 10;
        let register = match fsel {
            0 => FSEL0,
            1 => FSEL1,
            2 => FSEL2,
            _ => panic!("ERROR: Couldn't match to a function select register"),
        };

        let mut val: u32;

        unsafe {
            val = core::ptr::read_volatile(register as *mut u32);
        }

        let bit_position = (pin % 10) * 3;
        val &= !(0b111 << bit_position);
        val |= 0b000 << bit_position;

        unsafe {
            core::ptr::write_volatile(register as *mut u32, val);
        }
    }

    pub fn set_pin_to_high(pin: u32) {
        let bitpos = pin;

        let mut val: u32;

        unsafe {
            val = core::ptr::read_volatile(GPIO_SET0 as *mut u32);
        }

        val |= 1 << bitpos;

        unsafe {
            core::ptr::write_volatile(GPIO_SET0 as *mut u32, val);
        }
    }

    pub fn set_pin_to_low(pin: u32) {
        let bitpos = pin;

        let mut val: u32;

        unsafe {
            val = core::ptr::read_volatile(GPIO_CLEAR0 as *mut u32);
        }

        val |= 1 << bitpos;

        unsafe {
            core::ptr::write_volatile(GPIO_CLEAR0 as *mut u32, val);
        }
    }

    pub fn get_pin_state(pin: u32) -> bool {
        let val: u32;

        unsafe {
            val = core::ptr::read_volatile(GPIO_PIN_LEVEL0 as *mut u32);
        }

        val & (1 << pin) != 0
    }
}

#[link_section = ".text._start"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    GPIO::set_pin_to_output(12);
    GPIO::set_pin_to_input(26);

    loop {
        if GPIO::get_pin_state(26) {
            GPIO::set_pin_to_high(12);
        } else {
            GPIO::set_pin_to_low(12);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
