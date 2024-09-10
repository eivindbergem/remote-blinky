use embedded_hal::{delay::DelayNs, digital::InputPin};
use nrf52840_hal::{
    gpio::{Input, Pin, PullUp},
    gpiote::Gpiote,
};

pub struct Button {
    gpiote: Gpiote,
    pin: Pin<Input<PullUp>>,
}

pub enum Event {
    Pushed,
    Released,
}

impl Button {
    pub fn new(pin: Pin<Input<PullUp>>, gpiote: Gpiote) -> Self {
        // Add new event on any change of pin state
        gpiote.channel0().input_pin(&pin).toggle();

        Self { gpiote, pin }
    }

    pub fn debounced_event<T: DelayNs>(&mut self, timer: &mut T) -> Option<Event> {
        // Check for event
        if self.gpiote.channel0().is_event_triggered() {
            let mut i = 0;
            let mut is_high = self.pin.is_high().unwrap();

            // Debounce signal
            while i < 10 {
                // Add short delay between samples
                timer.delay_ms(1);

                let new_value = self.pin.is_high().unwrap();

                if new_value != is_high {
                    is_high = new_value;
                    i = 0;
                } else {
                    i += 1;
                }
            }

            // Reset all events that happend during debounce
            self.gpiote.reset_events();

            let event = if is_high {
                Event::Released
            } else {
                Event::Pushed
            };

            Some(event)
        } else {
            None
        }
    }
}
