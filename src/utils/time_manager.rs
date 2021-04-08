pub const DISPLAY_REFRESH: u64 = 16;
pub const TIMER_TICK: u64 = 1;

use std::{result::Result, thread, time::Duration, 
    sync::{Arc, Mutex, mpsc::{Sender, Receiver, channel}}};


#[derive(Copy, Clone, PartialEq)]
pub enum TimeTo {
    Update,
    Sleep,
}

pub struct TimeManager {
    timer: u64,
    sender: Sender<TimeTo>,
}

impl TimeManager {
    pub fn new(new_sender: Sender<TimeTo>) -> TimeManager {
        TimeManager {
            timer: 0,
            sender: new_sender,
        }
    }

    pub fn start_clock(&mut self) {
        loop {
            self.tick();
            if self.timer % DISPLAY_REFRESH == 0 {
                self.sender.send(TimeTo::Update).unwrap_or(());
                self.timer = 0;
            }

            thread::sleep(Duration::from_millis(TIMER_TICK));
        }
    }

    fn tick(&mut self) {
        self.timer += TIMER_TICK;
    }
}