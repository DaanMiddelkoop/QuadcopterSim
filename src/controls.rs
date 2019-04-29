pub struct Controller {
    pub lf: f32,
    pub rf: f32,
    pub lb: f32,
    pub rb: f32,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            lf: 1.0,
            rf: 1.0,
            lb: 0.0,
            rb: 0.0,
        }
    }

    pub fn update(&mut self) {

    }
}