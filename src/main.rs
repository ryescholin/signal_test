use std::thread;
use std::time::Duration;
use rppal::gpio::{Gpio, OutputPin};

const GPIO_PIN: u8 = 21;

fn main() {
    // Initialize the GPIO
    let gpio = Gpio::new().unwrap();
    let mut pin = gpio.get(GPIO_PIN).unwrap().into_output();

    // Blink the LED
    loop {
        pin.set_high();
        thread::sleep(Duration::from_millis(500)); // Wait for 0.5 seconds
        pin.set_low();
        thread::sleep(Duration::from_millis(500)); // Wait for 0.5 seconds
    }
}
