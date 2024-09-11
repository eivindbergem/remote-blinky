# Remote blinky on nRF52 in Rust

Code from [presentation at NDC TechTown
2024](https://ndctechtown.com/agenda/remote-blinky-on-nrf52-using-rust-0lc1/09vornw2mkh).

## Requirements

- nRF52840
- [Rust](https://www.rust-lang.org/tools/install)
- `rustup target add thumbv7em-none-eabihf`
- `cargo install probe-rs-tools --locked`

## Pinout

| Pin | Function |
| --- | --- | 
| P0_03 | LED |
| P0_26| Button|

## Usage

Connect debug-probe and run:

`cargo embed`

