#![no_std]
#![no_main]

extern crate panic_halt;


use embedded_graphics::image::Image16BPP;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::rectangle::Rectangle;

use riscv_rt::entry;
use gd32vf103_hal as hal;
use hal::prelude::*;
use hal::pac as pac;
use hal::spi::{Spi, Mode, Polarity, Phase};
use hal::delay;
use hal::ctimer;
use embedded_hal::blocking::delay::DelayMs;
use st7735_lcd;
use st7735_lcd::Orientation;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcu.apb2);
    let clocks = rcu.clocks;
    let ctimer = ctimer::CoreTimer::new(dp.CTIMER);;
    let mut delay = delay::Delay::new(clocks, ctimer);
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.ctl0);
    let miso = gpioa.pa6.into_floating_input(&mut gpioa.ctl0);
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.ctl0);
    let nss = gpioa.pa4.into_alternate_push_pull(&mut gpioa.ctl0);
    let spi_mode = Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition
    };
    let spi = Spi::spi0(
                        dp.SPI0,
                        (sck,miso,mosi,nss),
                        spi_mode,
                        1.mhz(),
                        clocks,
                        &mut rcu.apb2
                      );
    let tft_rst_pin = gpiob.pb1.into_push_pull_output(&mut gpiob.ctl0);
    let tft_rs_pin = gpiob.pb0.into_push_pull_output(&mut gpiob.ctl0); // Data or command select
    let tft_cs_pin = gpiob.pb2.into_push_pull_output(&mut gpiob.ctl0);
    // let mut spi = hal::spi()
    let mut disp = st7735_lcd::ST7735::new(spi, tft_rs_pin, tft_rst_pin,true, true);
    disp.init(&mut delay).unwrap();
    disp.set_orientation(&Orientation::Landscape).unwrap();
    let black_backdrop = Rectangle::new(Coord::new(0, 0), Coord::new(160, 80)).fill(Some(0x0000u16.into()));

    disp.draw(black_backdrop.into_iter());

    let ferris = Image16BPP::new(include_bytes!("./ferris.raw"), 86, 64).translate(Coord::new(40, 33));

    disp.draw(ferris.into_iter());

    loop {}
}
