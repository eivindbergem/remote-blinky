use embedded_hal::digital::OutputPin;

pub struct Led<P> {
    pin: P,
}

impl<P: OutputPin> Led<P> {
    pub fn new(pin: P) -> Self {
        Self { pin }
    }

    pub fn on(&mut self) {
        self.pin.set_high().unwrap();
    }

    pub fn off(&mut self) {
        self.pin.set_low().unwrap();
    }
}
