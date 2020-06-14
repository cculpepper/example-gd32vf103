#![no_std]
#![no_main]

extern crate panic_halt;


use riscv_rt::entry;
use gd32vf103_hal as hal;
use hal::prelude::*;
use hal::pac as pac;
use pac::TIMER4;
use hal::delay;
use hal::ctimer;
use hal::spi::{Spi, Mode, Polarity, Phase};
// use hal::pwm::Pwm;
use hal::rcu;
use hal::timer::Timer;
use embedded_hal::blocking::delay::DelayMs;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcu.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcu.apb2);
    // let mut afen = dp.APB2.split(&mut rcu.apb2);
    let mut pc13 = gpioc.pc13.into_push_pull_output(&mut gpioc.ctl1);
    let clocks = rcu::Strict::new().freeze(&mut rcu.cfg);
    let ctimer = ctimer::CoreTimer::new(dp.CTIMER);;
    let mut delay = delay::Delay::new(clocks, ctimer);
    let mut timer4 = dp.TIMER4;


    // Next goal is to set up a simple PWM on PA0. This is the VFD filament wire.
    // Datasheet says 50/60 Hz, so it probably can take ~48 to 64 ish...
    // Actually, we're good to push it over 10KHz. 
    // Timer 4, channel 0. This runs off the CK_TIMER clock
    //

    pc13.set_high().unwrap();
    delay.delay_ms(1000 as u32);

    // let mut pwm4ch1 = Some(gpioa.pa0.into_alternate_push_pull(&mut gpioa.ctl0));
    let mut pwm4ch2 = Some(gpioa.pa1.into_alternate_push_pull(&mut gpioa.ctl0));
    // let mut pwm4ch3 = Some(gpioa.pa2.into_alternate_push_pull(&mut gpioa.ctl0));
    // let mut pwm4ch4 = Some(gpioa.pa3.into_alternate_push_pull(&mut gpioa.ctl0));

    // Goal is to get PWM on PA1, green. timer4_ch1

    unsafe {
        rcu.apb1.en().modify(|_,w| w.timer4en().set_bit());
        timer4.psc.write(|w| w.psc().bits(10));
        timer4.ctl0.write(|w| w
                          .cam().bits(0b00) // Count up
                          .dir().clear_bit() // Edge aligned
                          .ckdiv().bits(0b00)
                         );

        timer4.car.write(|w| w.carl().bits(15999));

        timer4.swevg.write(|w| w.upg().set_bit());
        // Timer should theoretically be ticking. HOPEFULLY
        // Disable and reanable, clear and set polarity
        timer4.chctl2.modify(|_,w|
                             w.ch1en().clear_bit()
                             );

        timer4.chctl2.modify(|_,w|
                             w.ch1en().set_bit()
                             );

        timer4.chctl2.modify(|_,w|
                             w.ch1p().clear_bit()
                             );

        timer4.chctl2.modify(|_,w|
                             w.ch1p().set_bit()
                             );

        // Make sure the mode is set so chanel 1 is configured to be an output
        timer4.chctl0_output().modify(|_,w|
                             w.ch1ms().bits(0b00)
                             );


    timer4.ch1cv.write(|w| w.ch1val().bits(1999));
    timer4.chctl0_output().modify(|_,w| w.ch1comctl().bits(0b110));
    timer4.chctl0_output().modify(|_,w| w.ch1comsen().clear_bit());

    timer4.ctl0.modify(|_,w| w.arse().set_bit());

    timer4.ctl0.modify(|_,w| w.cen().set_bit());
    
        



    }

    // pa1 is green
    // pa2 is blue

    // let mut pwm = Pwm::pwm(1000.hz(), clocks, dp.TIMER4, &mut rcu.apb1, &mut rcu.apb2, pwm4ch1, pwm4ch2, pwm4ch3, pwm4ch4);
    // let mut pwm = Pwm::pwm(1000.hz(), clocks, dp.TIMER4, &mut rcu.apb1, &mut rcu.apb2, pwm4ch2);
    // pwm.init();
    // // pwm.set_duty(1,100);
    // pwm.set_duty(2,100);
    // pwm.set_duty(3,100);
    // pwm.set_duty(4,100);

    let mut curPwm:u16 = 0;
    loop {
        curPwm += 100;
        if (curPwm > 15000){
            curPwm = 0;
        }
        unsafe{
    timer4.ch1cv.write(|w| w.ch1val().bits(curPwm));

        }
        delay.delay_ms(10 as u32);
    }
}
