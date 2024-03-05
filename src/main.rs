use gpio::{GpioOut};
use std::thread;
use std::time;

const PIN_SIGNAL_LIGHT: u64 = 21;

struct SignalLight {
    gpio_pin: gpio::sysfs::SysFsGpioOutput,
}

impl SignalLight {
    fn new() -> Result<Self, gpio::Error> {
        let gpio_pin = gpio::sysfs::SysFsGpioOutput::open(PIN_SIGNAL_LIGHT)?;
        Ok(Self { gpio_pin })
    }

    fn disable(&mut self) -> Result<(), gpio::Error> {
        self.gpio_pin.set_value(false)?;
        Ok(())
    }

    fn enable(&mut self) -> Result<(), gpio::Error> {
        self.gpio_pin.set_value(true)?;
        Ok(())
    }
}

impl Drop for SignalLight {
    fn drop(&mut self) {
        if let Err(e) = self.disable() {
            eprintln!("Error cleaning up GPIO pin: {:?}", e);
        }
    }
}

fn main() {
    let mut signal_light = match SignalLight::new() {
        Ok(light) => light,
        Err(e) => {
            eprintln!("Error initializing SignalLight: {:?}", e);
            return;
        }
    };

    // Enable the signal light
    if let Err(e) = signal_light.enable() {
        eprintln!("Error enabling signal light: {:?}", e);
        return;
    }

    // Sleep for a while to demonstrate
    thread::sleep(time::Duration::from_secs(5));
}
