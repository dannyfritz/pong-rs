pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    pub fn new() -> Paddle {
        Paddle {
            x: 0.0,
            y: 50.0,
            width: 5.0,
            height: 5.0,
        }
    }
}
