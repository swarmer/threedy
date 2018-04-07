use piston_window as pw;


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InputState {
    pub left: pw::ButtonState,
    pub right: pw::ButtonState,
    pub front: pw::ButtonState,
    pub back: pw::ButtonState,
    pub up: pw::ButtonState,
    pub down: pw::ButtonState,
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            left: pw::ButtonState::Release,
            right: pw::ButtonState::Release,
            front: pw::ButtonState::Release,
            back: pw::ButtonState::Release,
            up: pw::ButtonState::Release,
            down: pw::ButtonState::Release,
        }
    }

    pub fn reset(&mut self) {
        self.left = pw::ButtonState::Release;
        self.right = pw::ButtonState::Release;
        self.front = pw::ButtonState::Release;
        self.back = pw::ButtonState::Release;
        self.up = pw::ButtonState::Release;
        self.down = pw::ButtonState::Release;
    }
}
