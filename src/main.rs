#![no_std]
#![no_main]

use panic_halt as _;
pub use cortex_m_rt::entry;
use stm32f3::stm32f303::{self, interrupt};

#[cortex_m_rt::entry]
fn start() -> ! {
    // Acquire the device peripherals. They can only be taken once ever.
    let device_peripherals = stm32f303::Peripherals::take().unwrap();

    // Get a reference to GPIOA and RCC to save typing.
    let gpioe = &device_peripherals.GPIOE;
    let rcc = &device_peripherals.RCC;
    let tim2 = &device_peripherals.TIM2;

    // Enable the GPIOA clock and set PA8 to be an output
    rcc.ahbenr.modify(|_, w| w.iopeen().enabled());
    gpioe.moder.modify(|_, w| w.moder8().output());

    // Set up the timer for slow interrupt generation
    // NOTE(unsafe): The psc field has not been sufficiently documented
    // to allow safe writing of arbitrary integer values, so we have to
    // use unsafe here. This could be fixed by improving the SVD file.
    rcc.apb1enr.modify(|_, w| w.tim2en().enabled());
    tim2.dier.write(|w| w.uie().enabled());
    tim2.psc.write(|w| w.psc().bits(1000));
    tim2.arr.write(|w| w.arr().bits(2000));
    tim2.cr1.write(|w| w.cen().enabled());

    // Enable the timer interrupt in the NVIC.
    unsafe { cortex_m::peripheral::NVIC::unmask(stm32f303::Interrupt::TIM2) };

    // The main thread can now go to sleep.
    // WFI (wait for interrupt) puts the core in sleep until an interrupt occurs.
    loop {
        cortex_m::asm::wfi();
    }
}

/// Interrupt handler for TIM2
#[interrupt]
fn TIM2() {
    // NOTE(unsafe): We have to use unsafe to access the peripheral
    // registers in this interrupt handler because we already used `take()`
    // in the main code. In this case all our uses are safe, not least because
    // the main thread only calls `wfi()` after enabling the interrupt, so
    // no race conditions or other unsafe behaviour is possible.
    // For ways to avoid using unsafe here, consult the Concurrency chapter:
    // https://rust-embedded.github.io/book/concurrency/concurrency.html

    // Clear the UIF bit to indicate the interrupt has been serviced
    unsafe { (*stm32f303::TIM2::ptr()).sr.modify(|_, w| w.uif().clear_bit()) };

    // Read ODR8 to see if the pin is set, and if so, clear it,
    // otherwise, set it. We use the atomic BSRR register to
    // set/reset it without needing to read-modify-write ODR.
    let ptr = stm32f303::GPIOE::ptr();
    unsafe {
        if (*ptr).odr.read().odr8().is_high() {
            (*ptr).bsrr.write(|w| w.br8().set_bit());
        } else {
            (*ptr).bsrr.write(|w| w.bs8().set_bit());
        }
    }
}
