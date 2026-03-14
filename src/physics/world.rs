#[derive(Default)]
// The PhysicsWorld struct represents the state of the physics simulation. It contains a step count that tracks how many times the simulation has been stepped. In a more complete implementation, this struct would also contain the bodies, forces, and other state needed to run the physics simulation.
pub struct PhysicsWorld {
    step_count: u64,
}

impl PhysicsWorld {
    pub fn step(&mut self, _dt: f32) {
        self.step_count += 1;
    }

    pub fn step_count(&self) -> u64 {
        self.step_count
    }
}
