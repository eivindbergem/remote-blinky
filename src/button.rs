use nrf52840_hal::{
    gpio::{Input, Pin, PullUp},
    gpiote::Gpiote,
};

pub struct Button {
    gpiote: Gpiote,
}

impl Button {
    pub fn new(pin: &Pin<Input<PullUp>>, gpiote: Gpiote) -> Self {
        gpiote.channel0().input_pin(pin).hi_to_lo();
        gpiote.channel1().input_pin(pin).lo_to_hi();

        Self { gpiote }
    }

    pub fn has_been_pushed(&self) -> bool {
        self.gpiote.channel0().is_event_triggered()
    }

    pub fn has_been_released(&self) -> bool {
        self.gpiote.channel1().is_event_triggered()
    }

    pub fn clear_events(&self) {
        self.gpiote.reset_events();
    }
}
