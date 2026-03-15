#[derive(Debug, Clone, Copy)]
pub enum Collider {
    Circle { radius: f32 },
    Aabb { half_extents: (f32, f32) },
}
