use embassy_nrf::gpio::{Input, Level};
use embassy_time::{with_timeout, Duration};
use garage_door_core::{DoorState, Source};

// NC reed sensor
const DOOR_CLOSED_LEVEL: Level = Level::High;

/// Reed sensor can trigger multiple times on garage door bounce
const SETTLE_TIME: Duration = Duration::from_millis(1000);

pub struct ReedSwitch<'d> {
    pin: Input<'d>,
}

impl<'d> ReedSwitch<'d> {
    pub fn new(pin: Input<'d>) -> Self {
        Self { pin }
    }
}

impl Source for ReedSwitch<'_> {
    type Event = DoorState;

    fn current(&mut self) -> DoorState {
        if self.pin.get_level() == DOOR_CLOSED_LEVEL {
            DoorState::Closed
        } else {
            DoorState::Opened
        }
    }

    async fn changed(&mut self) -> DoorState {
        self.pin.wait_for_any_edge().await;
        // 'debounce' the garage door 'bounce' on stop lol
        while with_timeout(SETTLE_TIME, self.pin.wait_for_any_edge())
            .await
            .is_ok()
        {}
        self.current()
    }
}