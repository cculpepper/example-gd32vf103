#![feature(asm)]
#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use gd32vf103_hal as hal;
use hal::prelude::*;
use hal::pac as pac;
use hal::delay;
use hal::ctimer;
use hal::systick::Systick;
use embedded_hal::blocking::delay::DelayMs;

#[entry]
fn main() -> !
{
     // pa1 G
     // pa2 B
     // pc13 R
    let mut sys : u64;
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcu.apb2);
    let mut pa1 = gpioa.pa1.into_push_pull_output(&mut gpioa.ctl0)
        .lock(&mut gpioa.lock);
    let mut pa2 = gpioa.pa2.into_push_pull_output(&mut gpioa.ctl0)
        .lock(&mut gpioa.lock);
    let mut pc13 = gpioc.pc13.into_push_pull_output(&mut gpioc.ctl1)
        .lock(&mut gpioc.lock);
    let clocks = rcu.clocks;
    let ctimer = ctimer::CoreTimer::new(dp.CTIMER);
    let mut delay = delay::Delay::new(clocks, ctimer);
    let mut i : u32;
    gpioa.lock.freeze();
    // let mut count : i32 = 32;
    // This will output a number
    // while (count > 0){
        // count -= 1;
        // if (sys & 1 == 0){
            // pa1.set_low().unwrap();
            // shittyDelay();
        // } else {
            // pa2.set_low().unwrap();
            // shittyDelay();
        // }
        // sys = sys >> 1;
        // pa1.set_high().unwrap();
            // pa2.set_high().unwrap();
        // shittyDelay();
        // shittyDelay();
        // shittyDelay();
    // }
    // pc13.set_low().unwrap();






    // ===========================================
    pa1.set_high().unwrap();
        // delay.delay_ms(1000 as u32);
    pa2.set_low().unwrap();
    pc13.set_high().unwrap();

    loop {
        pa2.toggle().unwrap();
        delay.delay_ms(1000 as u32);
    }
}
fn shittyDelay(){
        for i in 0..500000{
            unsafe { asm!("nop") };
        }

}
