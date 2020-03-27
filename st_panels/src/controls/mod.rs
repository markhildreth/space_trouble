mod four_switch;
mod push_button;
mod toggle_switch;

pub use four_switch::FourSwitch;
pub use push_button::PushButton;
pub use toggle_switch::ToggleSwitch;

pub enum UpdateResult<T> {
    NoChange,
    Change(T),
}

pub trait Control
where
    Self: Sized,
    <Self as Control>::Value: Default + PartialEq + Clone + Copy,
{
    type Value;

    fn stateful(self) -> StatefulControl<Self> {
        StatefulControl::new(self)
    }

    fn debounce(self, ms: u32) -> DebounceControl<Self> {
        DebounceControl::new(self, ms)
    }

    fn read(&self) -> Self::Value;
}

pub struct StatefulControl<T>
where
    T: Control,
    T::Value: Default,
{
    control: T,
    current_value: T::Value,
}

impl<T> StatefulControl<T>
where
    T: Control,
{
    fn new(control: T) -> StatefulControl<T> {
        StatefulControl {
            control,
            current_value: T::Value::default(),
        }
    }

    pub fn update(&mut self, _ms: u32) -> UpdateResult<T::Value> {
        let value = self.control.read();
        if value == self.current_value {
            UpdateResult::NoChange
        } else {
            self.current_value = value;
            UpdateResult::Change(self.current_value)
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum DebounceStatus<T> {
    Neutral,
    Debouncing { ends_at: u32, de_value: T },
}

pub struct DebounceControl<T>
where
    T: Control,
{
    control: T,
    debounce_time: u32,
    current_value: T::Value,
    debounce_status: DebounceStatus<T::Value>,
}

impl<T> DebounceControl<T>
where
    T: Control,
{
    fn new(control: T, debounce_time: u32) -> DebounceControl<T> {
        DebounceControl {
            control,
            debounce_time,
            current_value: T::Value::default(),
            debounce_status: DebounceStatus::Neutral,
        }
    }

    pub fn update(&mut self, ms: u32) -> UpdateResult<T::Value> {
        let new_value = self.control.read();
        match (self.debounce_status, self.current_value == new_value) {
            (DebounceStatus::Neutral, true) => UpdateResult::NoChange,
            (DebounceStatus::Neutral, false) => {
                self.start_debounce(ms, new_value);
                UpdateResult::NoChange
            }
            (DebounceStatus::Debouncing { .. }, true) => {
                self.stop_debouncing();
                UpdateResult::NoChange
            }
            (DebounceStatus::Debouncing { de_value, ends_at }, false) => {
                if de_value != new_value {
                    self.start_debounce(ms, new_value);
                    UpdateResult::NoChange
                } else if ms > ends_at {
                    self.current_value = de_value;
                    self.stop_debouncing();
                    UpdateResult::Change(self.current_value)
                } else {
                    UpdateResult::NoChange
                }
            }
        }
    }

    fn start_debounce(&mut self, ms: u32, value: T::Value) {
        self.debounce_status = DebounceStatus::Debouncing {
            de_value: value,
            ends_at: ms + self.debounce_time,
        };
    }

    fn stop_debouncing(&mut self) {
        self.debounce_status = DebounceStatus::Neutral;
    }
}