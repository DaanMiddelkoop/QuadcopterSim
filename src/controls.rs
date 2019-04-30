pub struct Controller {
    pub lf: f64,
    pub rf: f64,
    pub lb: f64,
    pub rb: f64,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            lf: 0.0,
            rf: 1.0,
            lb: 0.0,
            rb: 0.0,
        }
    }

    pub fn update(&mut self) {

    }
}