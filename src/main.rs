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
    ieee802154::{Packet, Radio, TxPower},
    timer::Periodic,
    Clocks, Timer,
};
use nrf52840_pac::TIMER1;
use rtt_target::{rprintln, rtt_init_print};

use panic_rtt_target as _;

mod button;
mod led;

const LED_ON: &[u8] = b"\xba\xbe\xfa\xce";
const LED_OFF: &[u8] = b"\xde\xad\xbe\xef";

enum Event {
    CrcError,
    ButtonPushed,
    PacketReceived,
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    let p = hal::pac::Peripherals::take().unwrap();

    let clocks = Clocks::new(p.CLOCK).enable_ext_hfosc();

    let mut radio = Radio::init(p.RADIO, &clocks);
    radio.set_txpower(TxPower::Pos8dBm);

    let mut timer = Timer::periodic(p.TIMER1);

    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let mut led = Led::new(port0.p0_03.into_push_pull_output(Level::Low));
    let button_pin = port0.p0_26.into_pullup_input().degrade();

    let button = Button::new(&button_pin, Gpiote::new(p.GPIOTE));

    rprintln!("Remote blinky started");

    let mut packet = Packet::new();

    loop {
        rprintln!("Waiting for event");

        let event = radio.recv_non_blocking(&mut packet, |recv| loop {
            if button.has_been_pushed() {
                rprintln!("Button pushed");
                break Event::ButtonPushed;
            }

            match recv.is_done() {
                Ok(_) => break Event::PacketReceived,
                Err(nb::Error::WouldBlock) => continue,

                // We have to restart transfer in case of CRC error
                Err(nb::Error::Other(_)) => break Event::CrcError,
            }
        });

        match event {
            Event::ButtonPushed => {
                packet.copy_from_slice(LED_ON);

                radio.send(&mut packet);

                timer.delay(Timer::<TIMER1, Periodic>::TICKS_PER_SECOND);

                rprintln!("Waiting for release");
                while !button.has_been_released() {
                    asm::nop();
                }

                packet.copy_from_slice(LED_OFF);

                radio.send(&mut packet);

                button.clear_events();
            }
            Event::PacketReceived => {
                rprintln!("Received packet: {:X?}", packet.deref());

                if packet.deref() == LED_ON {
                    rprintln!("Turning on LED");
                    led.on();

                    while radio.recv(&mut packet).is_err() || packet.deref() != LED_OFF {
                        asm::nop();
                    }

                    rprintln!("Turning off LED");
                    led.off();
                }
            }
            Event::CrcError => (),
        }
    }
}
