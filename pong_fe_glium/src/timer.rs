use time;
#[derive(Debug)]
pub struct Timer {
    pub dt: f64,
    pub current_time: f64,
    pub accumulator: f64,
}

impl Timer {
    pub fn new(dt: f64) -> Timer {
        Timer {
            dt: dt,
            current_time: time::precise_time_s(),
            accumulator: 0.0,
        }
    }
    pub fn tick(&mut self) {
        let new_time = time::precise_time_s();
        let frame_time = new_time - self.current_time;
        self.current_time = new_time;
        self.accumulator += frame_time;
    }
    pub fn drain(&mut self) -> bool {
        if self.accumulator >= self.dt {
            self.accumulator -= self.dt;
            true
        } else {
            false
        }
    }
}
