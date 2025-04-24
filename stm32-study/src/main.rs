#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

// use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f103_pac as pac;

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    configure_clock(&peripherals);

    let timer2 = configure_timer2(&peripherals);
    let gpioa = configure_gpioa_output(&peripherals);
    let gpiob = configure_gpiob_input(&peripherals);
    // let gpioc = configure_gpioc_output(&peripherals);

    // let delay = 1000;

    let mut led_on = false;

    loop {
        // gpioc.bsrr().write(|w| w.br13().set_bit());
        // delay_ms(100, &timer2);
        // gpioc.bsrr().write(|w| w.bs13().set_bit());
        // delay_ms(1000, &timer2);


        // gpioa.bsrr().write(|w| w.bs3().set_bit());
        // delay_ms(100, &timer2);
        // gpioa.bsrr().write(|w| w.br3().set_bit());
        // delay_ms(100, &timer2);
        // gpioa.bsrr().write(|w| w.bs3().set_bit());
        // delay_ms(100, &timer2);
        // gpioa.bsrr().write(|w| w.br3().set_bit());
        // delay_ms(1000, &timer2);

        if gpiob.idr().read().idr12().bit_is_set() {
            led_on = !led_on;
        }

        if led_on {
            gpioa.bsrr().write(|w| w.bs3().set_bit());
        } else {
            gpioa.bsrr().write(|w| w.br3().set_bit());
        }
    }
}

fn configure_clock(peripherals: &pac::Peripherals) {
    // use external clock to 8 MHz
    peripherals
        .RCC
        .cr()
        .write(|w| w.hsion().clear_bit().hseon().set_bit());
    // wait HSE ready
    while !peripherals.RCC.cr().read().hserdy().bit() {}

    // set flash to works between 48MHz e 72MHz
    peripherals
        .FLASH
        .acr()
        .write(|w| unsafe { w.latency().bits(0b010) });

    // define pll source as HSE and multiply for 9
    peripherals
        .RCC
        .cfgr()
        .modify(|_, w| unsafe { w.pllsrc().set_bit().pllmul().bits(0b0111) });

    // Enable PLL
    peripherals.RCC.cr().modify(|_, w| w.pllon().set_bit());
    // wait PLL is ready
    while !peripherals.RCC.cr().read().pllrdy().bit() {}

    // define system clock to use PLL
    peripherals
        .RCC
        .cfgr()
        .modify(|_, w| unsafe { w.sw().bits(0b10) });
    // wait for PLL selected
    while peripherals.RCC.cfgr().read().sws().bits() != 0b10 {}

    peripherals.RCC.cfgr().modify(|_, w| unsafe {
        w.ppre2().bits(0b000).ppre1().bits(0b111) // divide clock for 16 -> 4.5MHz
    });
}

fn configure_timer2(peripherals: &pac::Peripherals) -> &pac::TIM2 {
    let timer = &peripherals.TIM2;

    peripherals
        .RCC
        .apb1enr()
        .modify(|_, w| w.tim2en().set_bit());

    timer.psc().write(|w| unsafe { w.psc().bits(2249) }); // 4.5MHz / 4500 = 2kHz
    timer.arr().write(|w| unsafe { w.bits(1) }); // 1ms por tick
    timer.cr1().modify(|_, w| w.cen().set_bit());

    &timer
}

fn configure_gpioa_output(peripherals: &pac::Peripherals) -> &pac::GPIOA {
    peripherals
        .RCC
        .apb2enr()
        .modify(|_, w| w.iopaen().set_bit());

    peripherals
        .GPIOA
        .crl()
        .write(|w| unsafe { w.mode3().bits(0b10).cnf3().bits(0b00) });

    &peripherals.GPIOA
}

fn configure_gpiob_input(peripherals: &pac::Peripherals) -> &pac::GPIOB {
    peripherals
        .RCC
        .apb2enr()
        .modify(|_, w| w.iopben().set_bit());

    peripherals
        .GPIOB
        .crh()
        .write(|w| unsafe { w.mode12().bits(0b00).cnf12().bits(0b10) });

    &peripherals.GPIOB
}

fn configure_gpioc_output(peripherals: &pac::Peripherals) -> &pac::GPIOC {
    peripherals
        .RCC
        .apb2enr()
        .modify(|_, w| w.iopcen().set_bit());

    peripherals
        .GPIOC
        .crh()
        .write(|w| unsafe { w.mode13().bits(0b10).cnf13().bits(0b00) });

    &peripherals.GPIOC
}

fn delay_ms(ms: u32, timer: &pac::TIM2) {
    let mut count: u32 = 0;
    while count < ms {
        if timer.sr().read().uif().bit_is_set() {
            timer.sr().modify(|_, w| w.uif().clear_bit());
            count += 1;
        }
    }
}
