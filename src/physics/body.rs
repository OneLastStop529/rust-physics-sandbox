use crate::{math::Vec2, physics::collider::Collider};

// Unique identifier for a body in the physics world
pub type BodyHandle = usize;

// Type of rigid body - dynamic bodies are affected by forces, while static
// bodies are not affected by forces and do not move
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyType {
    Dynamic,
    Static,
}

// Represents a rigid body in the physics simulation
// - `id`: Unique identifier for the body
// - `body_type`: Whether the body is dynamic (affected by forces) or static (
// not affected by forces)
// - `position`: Current position of the body in world space
// - `velocity`: Current velocity of the body
// - `force_accumulator`: Accumulated forces to be applied during the next update
// - `inverse_mass`: Inverse of the body's mass (0 for static bodies)
// - `restitution`: Coefficient of restitution (bounciness) for collisions
// - `friction`: Coefficient of friction for collisions
// - `collider`: Optional collider shape for collision detection
#[derive(Debug)]
pub struct RigidBody {
    pub id: u32,
    pub body_type: BodyType,
    pub position: Vec2,
    pub velocity: Vec2,
    pub force_accumulator: Vec2,
    pub inverse_mass: f32,
    pub restitution: f32,
    pub friction: f32,
    pub collider: Option<Collider>,
}

impl RigidBody {
    pub fn new(id: u32, body_type: BodyType, position: Vec2, collider: Option<Collider>) -> Self {
        let inverse_mass = match body_type {
            BodyType::Dynamic => 1.0,
            BodyType::Static => 0.0,
        };

        Self {
            id,
            body_type,
            position,
            velocity: Vec2::default(),
            force_accumulator: Vec2::default(),
            inverse_mass,
            restitution: 0.5,
            friction: 0.5,
            collider,
        }
    }
}
