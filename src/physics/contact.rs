use crate::{math::Vec2, physics::body::BodyHandle};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Contact {
    pub body_a: BodyHandle,
    pub body_b: BodyHandle,
    pub normal: Vec2,
    pub penetration: f32,
    pub point: Vec2,
    pub restitution: f32,
    pub friction: f32,
}
