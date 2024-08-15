#![no_main]
#![no_std]

use core::ops::Deref;

use button::Button;
use cortex_m::asm;
use led::Led;
use nrf52840_hal::{
    self as hal,
    gpio::Level,
    gpiote::Gpiote,
    ieee802154::{Packet, Radio},
    timer::Periodic,
    Clocks, Timer,
};
use nrf52840_pac::TIMER1;
use rtt_target::{rprintln, rtt_init_print};

use panic_rtt_target as _;

mod button;
mod led;

static LED_ON: &[u8] = &[0xba, 0xbe, 0xfa, 0xce];
static LED_OFF: &[u8] = &[0xde, 0xad, 0xbe, 0xef];

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    let p = hal::pac::Peripherals::take().unwrap();

    let clocks = Clocks::new(p.CLOCK).enable_ext_hfosc();

    let mut radio = Radio::init(p.RADIO, &clocks);
    let mut timer = Timer::periodic(p.TIMER1);

    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let mut led = Led::new(port0.p0_08.into_push_pull_output(Level::High));
    let button_pin = port0.p0_23.into_floating_input().degrade();

    let button = Button::new(&button_pin, Gpiote::new(p.GPIOTE));

    rprintln!("Remote blinky started");

    let mut radio_timer = Timer::one_shot(p.TIMER0);

    loop {
        if button.has_been_pushed() {
            let mut packet = Packet::new();
            packet.copy_from_slice(LED_ON);

            for _ in 0..3 {
                radio.send(&mut packet);
            }

            timer.delay(Timer::<TIMER1, Periodic>::TICKS_PER_SECOND);

            while !button.has_been_released() {
                asm::nop();
            }

            packet.copy_from_slice(LED_OFF);

            for _ in 0..3 {
                radio.send(&mut packet);
            }

            button.clear_events();
        }

        let mut packet = Packet::new();

        if radio
            .recv_timeout(&mut packet, &mut radio_timer, 1000 * 1000)
            .is_ok()
            && packet.deref() == LED_ON
        {
            rprintln!("Received packet: {:X?}", packet.deref());
            led.on();

            while radio.recv(&mut packet).is_err() || packet.deref() != LED_OFF {
                asm::nop();
            }

            led.off();
        }
    }
}
