#![no_std]
#![no_main]

use panic_rtt_target as _;
use rtic::app;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::gpio::PinState;
use stm32f1xx_hal::gpio::{gpioa::PA6, gpioc::PC13, Output, PushPull};
use stm32f1xx_hal::prelude::*;
use systick_monotonic::{fugit::Duration, Systick};

#[app(device = stm32f1xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: PC13<Output<PushPull>>,
        led2: PA6<Output<PushPull>>,
        state: bool,
    }

    #[monotonic(binds = SysTick, default = true)]
    type MonoTimer = Systick<1000>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        // setup clocks
        let mut flash = cx.device.FLASH.constrain();
        let rcc = cx.device.RCC.constrain();

        let mono = Systick::new(cx.core.SYST, 36_000_000);

        rtt_init_print!();
        rprintln!("init");

        let _clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(36.MHz())
            .pclk1(36.MHz())
            .freeze(&mut flash.acr);

        // setup LED
        let mut gpioc = cx.device.GPIOC.split();
        let mut gpioa = cx.device.GPIOA.split();

        let led = gpioc
            .pc13
            .into_push_pull_output_with_state(&mut gpioc.crh, PinState::Low);

        let led2 = gpioa
            .pa6
            .into_push_pull_output_with_state(&mut gpioa.crl, PinState::High);

        blink::spawn_after(Duration::<u64, 1, 1000>::from_ticks(500)).unwrap();

        (
            Shared {},
            Local {
                led,
                led2,
                state: false,
            },
            init::Monotonics(mono),
        )
    }

    #[task(local = [led, led2, state])]
    fn blink(cx: blink::Context) {
        rprintln!("blink");
        if *cx.local.state {
            cx.local.led.set_high();
            cx.local.led2.set_high();
            *cx.local.state = false;
        } else {
            cx.local.led.set_low();
            cx.local.led2.set_low();
            *cx.local.state = true;
        }

        blink::spawn_after(Duration::<u64, 1, 1000>::from_ticks(500)).unwrap();
    }
}
