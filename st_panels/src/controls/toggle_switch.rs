use crate::controls::Control;
use crate::{Pin, PinValue};
use st_data::control_values::ToggleSwitchValue;

pub struct ToggleSwitch<P: Pin> {
    pin: P,
}

impl<P: Pin> ToggleSwitch<P> {
    pub fn new(pin: P) -> ToggleSwitch<P> {
        ToggleSwitch { pin }
    }
}

impl<P: Pin> Control for ToggleSwitch<P> {
    type Value = ToggleSwitchValue;

    fn read(&self) -> Self::Value {
        match self.pin.read() {
            PinValue::Low => ToggleSwitchValue::Disabled,
            PinValue::High => ToggleSwitchValue::Enabled,
        }
    }
}