extern crate three;

mod cube;
mod vector; 
mod quadcopter;
mod controls;

use three::Object;
use three::controls::Timer;

use quadcopter::QuadCopter;

fn main() {
    let title = "Getting started with three-rs";
    let mut window = three::Window::new(title);

    let mut quadcopter = QuadCopter::new(0.0, 0.0, 0.0, &mut window);


    let center = [0.0, 0.0];
    let camera = window.factory.perspective_camera(60.0, 0.1 ..);
    camera.set_position([-5.0, -5.0, 5.0]);
    camera.look_at([-5.0,-5.0, 5.0], [0.0, 0.0, 0.0], None);

    let mut timer = Timer::new();

    while window.update() {
        let dt = timer.elapsed() as f64;
        timer.reset();

        for _ in 0..100 {
            quadcopter.update(dt / 100.0);
        }
        quadcopter.draw();
        window.render(&camera);
    }
}