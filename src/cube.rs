use three::Mesh;
use three::Object;

use crate::vector::Vector3f;

pub struct Cube {
    pub mesh: three::Mesh,
}

impl Cube {
    pub fn new(x: f64, y: f64, z: f64, color: u32, window: &mut three::Window) -> Cube {
        
        let geometry = three::Geometry::cuboid(x as f32, y as f32, z as f32);
        let material = three::material::Basic {
            color: color,
            .. Default::default()
        };
        let mesh = window.factory.mesh(geometry, material);
        
        window.scene.add(&mesh);

        Cube {
            mesh: mesh
        }
    }

    pub fn update(&mut self, position: &Vector3f) {
        self.mesh.set_position([position.x as f32, position.y as f32, position.z as f32]);
    }
}