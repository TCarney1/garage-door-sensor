#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive, Input, Pull};
use embassy_time::{with_timeout, Duration};
use panic_halt as _;

// LED is reverse polarity
const LED_ON: Level = Level::Low;
const LED_OFF: Level = Level::High;

// Reed switch is 'normally closed'.
// So door is closed when switch is open
const DOOR_CLOSED_LEVEL: Level = Level::High;

#[derive(Clone, Copy)]
pub enum DoorState {
    Opened,
    Closed
}

#[allow(async_fn_in_trait)]
pub trait Notifier {
    async fn notify(&mut self, state: DoorState);
}

struct LedNotifier<'d> {
    led: Output<'d>
}

impl Notifier for LedNotifier<'_> {
    async fn notify(&mut self, state: DoorState) {
        match state {
            DoorState::Opened => self.led.set_level(LED_ON),
            DoorState::Closed => self.led.set_level(LED_OFF),
        }
    }
}

impl<'d> LedNotifier<'d> {
    fn new(led: Output<'d>) -> Self {
        Self { led }
    }
}


fn read_state(sensor: &Input<'_>) -> DoorState {
    if sensor.get_level() == DOOR_CLOSED_LEVEL {
        DoorState::Closed
    } else {
        DoorState::Opened
    }
}

async fn watch_door<N: Notifier>(mut sensor: Input<'_>, mut out: N) {
    out.notify(read_state(&sensor)).await;

    loop {
        sensor.wait_for_any_edge().await;

        // 'debounce' the garage door 'bounce' on stop lol
        while with_timeout(Duration::from_millis(1000), sensor.wait_for_any_edge())
            .await
            .is_ok(){}

        out.notify(read_state(&sensor)).await;
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let led = Output::new(p.P0_15, Level::Low, OutputDrive::Standard);
    let sensor = Input::new(p.P0_02, Pull::Up);
    let led_notifier = LedNotifier::new(led);
    watch_door(sensor, led_notifier).await;
}