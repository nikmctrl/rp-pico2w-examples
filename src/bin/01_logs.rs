//! Start here. This example tests the RP Pico2 W USB serial port logging.
//! This demo starts up and launches a logging task to capture logging messages and to send them over the USB serial port.
//! It then waits for the host (the pc connected to the rp) to connect and start listening to text coming from the serial port
//! Then it logs a counter every second to show you that it is working. If you do not see any log messages after the program
//! has finished downloading then there is a communication problem.
//!
//! How to run with a standard usb cable (no debug probe):
//! The pico has a built-in bootloader that can be used as a replacement for a debug probe (like an ST link v2 or a JLINK). This demo uses the bootloader.
//! Start with the usb cable unplugged then, while holding down the BOOTSEL button, plug it in. Then you can release the button.
//! Mount the usb drive (this will be enumerated as USB mass storage) then run the following command:
//! cargo run --bin 01_logs --release
//!
//!
//! Troubleshoot:
//! `Error: "Unable to find mounted pico"`
//! This is because the pico is not in bootloader mode. You need to press down the BOOTSEL button when you plug it in and then release the button.
//! You need to do this every time you download firmware onto the device.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    block::ImageDef,
    peripherals::USB,
    usb::{self, Driver},
};
use embassy_time::{Duration, Timer};
use log::info;
use panic_halt as _;

// NOTE: panic messages are not sent over usb serial because the pico has to be in a somewhat decent state to send data over USB
// However, when it panics it stops the scheduler. In order to see panic messages we really need to use a probe

#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => usb::InterruptHandler<USB>;
});

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // setup logging over usb serial port
    let driver = Driver::new(p.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    // wait for host to connect to usb serial port
    Timer::after(Duration::from_secs(1)).await;
    info!("started");
    info!("if using screen on Linux press 'Ctrl-a d' to quit");

    let mut counter = 0;
    loop {
        info!("count: {}", counter);
        counter += 1;
        Timer::after(Duration::from_secs(1)).await;
    }
}
