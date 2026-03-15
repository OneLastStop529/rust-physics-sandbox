use crate::{
    math::Vec2,
    physics::body::{BodyType, RigidBody},
};

pub fn acceleration(body: &RigidBody, gravity: Vec2) -> Vec2 {
    match body.body_type {
        BodyType::Dynamic => gravity + (body.force_accumulator * body.inverse_mass),
        BodyType::Static => Vec2::default(),
    }
}

// Integrates the body's position and velocity using semi-implicit Euler integration.
pub fn integrate_body(body: &mut RigidBody, dt: f32, gravity: Vec2) {
    if body.is_static() {
        body.velocity = Vec2::default();
        body.force_accumulator = Vec2::default();
        return;
    }

    let acceleration = acceleration(body, gravity);
    body.velocity += acceleration * dt;
    body.position += body.velocity * dt;
    body.force_accumulator = Vec2::default();
}

#[cfg(test)]
mod tests {
    use super::{acceleration, integrate_body};
    use crate::{
        math::Vec2,
        physics::{
            body::{BodyType, RigidBody},
            collider::Collider,
        },
    };

    fn body(body_type: BodyType) -> RigidBody {
        RigidBody::new(
            0,
            body_type,
            Vec2::new(0.0, 0.0),
            Some(Collider::Circle { radius: 1.0 }),
        )
    }

    #[test]
    fn combines_gravity_and_forces_for_dynamic_bodies() {
        let mut body = body(BodyType::Dynamic);
        body.inverse_mass = 0.5;
        body.force_accumulator = Vec2::new(4.0, 10.0);

        let acceleration = acceleration(&body, Vec2::new(0.0, -9.0));

        assert_eq!(acceleration, Vec2::new(2.0, -4.0));
    }

    #[test]
    fn static_bodies_have_no_acceleration() {
        let mut body = body(BodyType::Static);
        body.force_accumulator = Vec2::new(10.0, -3.0);

        let acceleration = acceleration(&body, Vec2::new(0.0, -9.0));

        assert_eq!(acceleration, Vec2::default());
    }

    #[test]
    fn uses_semi_implicit_euler_for_dynamic_bodies() {
        let mut body = body(BodyType::Dynamic);
        body.velocity = Vec2::new(1.0, 0.0);
        body.force_accumulator = Vec2::new(2.0, 0.0);

        integrate_body(&mut body, 0.5, Vec2::new(0.0, -4.0));

        assert_eq!(body.velocity, Vec2::new(2.0, -2.0));
        assert_eq!(body.position, Vec2::new(1.0, -1.0));
        assert_eq!(body.force_accumulator, Vec2::default());
    }

    #[test]
    fn leaves_static_bodies_stationary() {
        let mut body = body(BodyType::Static);
        body.velocity = Vec2::new(3.0, -2.0);
        body.force_accumulator = Vec2::new(5.0, 1.0);

        integrate_body(&mut body, 1.0, Vec2::new(0.0, -9.81));

        assert_eq!(body.position, Vec2::default());
        assert_eq!(body.velocity, Vec2::default());
        assert_eq!(body.force_accumulator, Vec2::default());
    }
}
