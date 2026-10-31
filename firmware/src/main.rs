#![no_std]
#![no_main]

mod led;
mod reed;

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Output, OutputDrive, Pull};
use garage_door_core::forward;
use panic_halt as _;

use led::{LedNotifier, LED_OFF};
use reed::ReedSwitch;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    let led = Output::new(p.P0_15, LED_OFF, OutputDrive::Standard);
    let sensor = Input::new(p.P0_02, Pull::Up);

    forward(ReedSwitch::new(sensor), LedNotifier::new(led)).await;
}