pub const DISPLAY_REFRESH: u128 = 17;
pub const OPCODE_REFRESH: u128 = 2000;
pub const TIMER_TICK: u64 = 100;

use std::{result::Result, thread, time::{Duration, Instant}, 
    sync::{Arc, Mutex, mpsc::{Sender, Receiver, channel}}};


#[derive(Copy, Clone, PartialEq)]
pub enum TimeTo {
    Update,
    Sleep,
    Process, 
}

pub struct TimeManager {
    time: Instant,
    instruction_time: Instant,
    sender: Sender<TimeTo>,
    speed: u128,
}

impl TimeManager {
    pub fn new(new_sender: Sender<TimeTo>) -> TimeManager {
        TimeManager {
            time: Instant::now(),
            instruction_time: Instant::now(),
            sender: new_sender,
            speed: OPCODE_REFRESH,
        }
    }

    pub fn start_clock(&mut self) {
        loop {
            if self.time.elapsed().as_millis() > DISPLAY_REFRESH {
                self.sender.send(TimeTo::Update).unwrap();
                self.time = Instant::now();
            } else if self.time.elapsed().as_millis() > self.speed {
                self.sender.send(TimeTo::Process).unwrap();
                self.instruction_time = Instant::now();
            }

            if self.time.elapsed().as_millis() > 10000000 {
                self.tick();
            }

            thread::sleep(Duration::from_micros(TIMER_TICK));
        }
    }

    pub fn check_time(&mut self) -> TimeTo {
        //println!("Elapsed time: {}", self.time.elapsed().as_millis());
        if self.time.elapsed().as_millis() > DISPLAY_REFRESH {
            self.time = Instant::now();
            return TimeTo::Update;
        } else if self.time.elapsed().as_micros() > self.speed {
            self.instruction_time = Instant::now();
            return TimeTo::Process;
        }

        if self.time.elapsed().as_millis() > 10000000 {
            self.tick();
        }

        return TimeTo::Sleep;
    }

    pub fn set_speed(&mut self, speed: u128) {
        self.speed = speed;
    }

    fn tick(&mut self) {
        self.time = Instant::now();
    }
}