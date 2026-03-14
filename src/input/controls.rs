use macroquad::prelude::{KeyCode, is_key_pressed};

// Represents the current state of the controls. This is used to determine whether the simulation should be paused, stepped, or quit.
#[derive(Clone, Copy, Default)]
pub struct ControlState {
    pub pause_toggled: bool,
    pub quit_requested: bool,
    pub single_step_requested: bool,
}

impl ControlState {
    // Polls the current state of the controls. This should be called once per frame, and the resulting ControlState should be passed to the TimingState to update the simulation state.
    pub fn poll() -> Self {
        Self {
            pause_toggled: is_key_pressed(KeyCode::Space),
            quit_requested: is_key_pressed(KeyCode::Escape),
            single_step_requested: is_key_pressed(KeyCode::N),
        }
    }
}
