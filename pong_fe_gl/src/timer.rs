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
}
