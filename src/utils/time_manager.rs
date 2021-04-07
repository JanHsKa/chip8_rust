pub const DISPLAY_REFRESH: u16 = 16;


pub enum TimeTo {
    Update,
    Sleep,
}

pub struct TimeManager {
    timer: u64,
}

impl TimeManager {
    pub fn new() -> TimeManager {
        TimeManager {
            timer: 0,
        }
    }

    pub fn tick(&mut self) {
        self.timer += 1;
    }
}