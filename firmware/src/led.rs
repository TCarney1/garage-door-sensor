use embassy_nrf::gpio::{Level, Output};
use garage_door_core::{DoorState, Notifier};

// LED is reverse polarity
pub const LED_ON: Level = Level::Low;
pub const LED_OFF: Level = Level::High;

pub struct LedNotifier<'d> {
    led: Output<'d>,
}

impl<'d> LedNotifier<'d> {
    pub fn new(led: Output<'d>) -> Self {
        Self { led }
    }
}

impl Notifier<DoorState> for LedNotifier<'_> {
    async fn notify(&mut self, state: DoorState) {
        match state {
            DoorState::Opened => self.led.set_level(LED_ON),
            DoorState::Closed => self.led.set_level(LED_OFF),
        }
    }
}