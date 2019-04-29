use crate::vector::Vector3f;
use crate::cube::Cube;
use crate::controls::Controller;

const ENGINE_THRUST: f32 = 100.0; // newton

struct Corner {
    pos: Vector3f,
    force: Vector3f,
    mass: f32,
    cube: Cube,
}

impl Corner {
    fn new(x: f32, y: f32, z: f32, color: u32, window: &mut three::Window) -> Corner {

        Corner {
            pos: Vector3f::new(x, y, z),
            force: Vector3f::new(0.0, 0.0, 0.0),
            mass: 1.0,
            cube: Cube::new(0.1, 0.1, 0.1, color, window),
        }
    }

    fn calc_grav_force(&mut self) {
        self.force.x = 0.0;
        self.force.y = 0.0;
        self.force.z = -9.81 * self.mass;
    }

    fn draw(&mut self) {
        self.cube.update(&self.pos);
    }
}



pub struct QuadCopter {
    lf: Corner,
    rf: Corner,
    lb: Corner,
    rb: Corner,
    vel: Vector3f,
    ang_vel: Vector3f,
    controller: Controller,
}

impl QuadCopter {
    pub fn new(x: f32, y: f32, z: f32, window: &mut three::Window) -> QuadCopter {


        QuadCopter {
            lf: Corner::new(x + 0.5, y + 0.5, z, 0xFF0000, window),
            rf: Corner::new(x + 0.5, y - 0.5, z, 0x0000FF, window),
            lb: Corner::new(x - 0.5, y + 0.5, z, 0x00FF00, window),
            rb: Corner::new(x - 0.5, y - 0.5, z, 0xFFFF00, window),
            vel: Vector3f::new(0.0, 0.0, 0.0),
            ang_vel: Vector3f::new(0.0, 0.0, 0.0),
            controller: Controller::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let total_mass = self.lf.mass + self.rf.mass + self.lb.mass + self.rb.mass;

        self.lf.calc_grav_force();
        self.rf.calc_grav_force();
        self.lb.calc_grav_force();
        self.rb.calc_grav_force();

        // Calculating direction of engine forces.
        // 2 vectors in the plane.
        let ab = self.rf.pos.sub(&self.lf.pos);
        let ac = self.lb.pos.sub(&self.lf.pos);

        // the normal vector of those 2 vectors (the vector perpendicular to the field):
        let normal = ab.cross(&ac).normalize().mult(-1.0);


        self.lf.force.add(&normal.mult(self.controller.lf * ENGINE_THRUST));
        self.rf.force.add(&normal.mult(self.controller.rf * ENGINE_THRUST));
        self.lb.force.add(&normal.mult(self.controller.lb * ENGINE_THRUST));
        self.rb.force.add(&normal.mult(self.controller.rb * ENGINE_THRUST));

        // calculating torque.
        let total_engine_power = (self.controller.lf + self.controller.rf + self.controller.lb + self.controller.rb);
        let lf_part = self.controller.lf / total_engine_power;
        let rf_part = self.controller.rf / total_engine_power;
        let lb_part = self.controller.lb / total_engine_power;
        let rb_part = self.controller.rb / total_engine_power;
        
        let rotational_force_position = self.lf.pos.mult(lf_part).add(&self.rf.pos.mult(rf_part).add(&self.lb.pos.mult(lb_part).add(&self.rb.pos.mult(rb_part))));
        let rotational_axis = self.lf.pos.add(&self.rf.pos.add(&self.lb.pos.add(&self.rb.pos))).mult(0.25);
        let relative_rotational_force_position = rotational_axis.sub(&rotational_force_position);
        

        println!("relative rotational force {} {} {}", relative_rotational_force_position.x, relative_rotational_force_position.y, relative_rotational_force_position.z);
        let total_engine_force = total_engine_power * ENGINE_THRUST;
        
        let mut torque = relative_rotational_force_position.cross(&normal.mult(total_engine_force));
        if total_engine_power == 0.0 {
            torque = Vector3f::new(0.0, 0.0, 0.0);
        }

        // t = I * a where t=torque, I = moment of inertia, a = angular velocity.
        // -> a = I * t
        // = m * r^2 * t
        println!("torque: {} {} {}", torque.x, torque.y, torque.z);

        


        let total_force = self.lf.force.add(&self.rf.force.add(&self.lb.force.add(&self.rb.force)));
        let accel = total_force.mult(1.0 / total_mass);

        self.vel = self.vel.add(&accel.mult(dt));

        self.lf.pos = self.lf.pos.add(&self.vel.mult(dt));
        self.rf.pos = self.rf.pos.add(&self.vel.mult(dt));
        self.lb.pos = self.lb.pos.add(&self.vel.mult(dt));
        self.rb.pos = self.rb.pos.add(&self.vel.mult(dt));


    }

    pub fn draw(&mut self) {
        self.lf.draw();
        self.rf.draw();
        self.lb.draw();
        self.rb.draw();

        ()
    }
}