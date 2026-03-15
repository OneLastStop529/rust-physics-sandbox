pub struct Contact {
    pub body_a: u32,
    pub body_b: u32,
    pub normal: (f32, f32),
    pub penetration: f32,
    pub point: (f32, f32),
    pub restitution: f32,
    pub friction: f32,
}
