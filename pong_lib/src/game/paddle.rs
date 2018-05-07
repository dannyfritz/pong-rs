pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
}

impl Paddle {
    pub fn new(x: f32) -> Paddle {
        Paddle {
            x: x,
            y: 16.0,
            width: 1.0,
            height: 8.0,
            speed: 20.0,
        }
    }
}
