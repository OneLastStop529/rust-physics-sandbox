use crate::{
    math::Vec2,
    physics::{
        body::{BodyHandle, BodyType, RigidBody},
        collider::Collider,
        collision::{CollisionStats, collect_collision_pairs, detect_collision},
        integrate::integrate_body,
    },
};

// Represents the physics simulation world, containing all bodies and managing the simulation state
pub struct PhysicsWorld {
    next_body_id: u32,
    step_count: u64,
    gravity: Vec2,
    bodies: Vec<RigidBody>,
    collision_stats: CollisionStats,
}

impl PhysicsWorld {
    pub fn new(gravity: Vec2) -> Self {
        Self {
            next_body_id: 0,
            step_count: 0,
            gravity,
            bodies: Vec::new(),
            collision_stats: CollisionStats::default(),
        }
    }

    pub fn step(&mut self, dt: f32) {
        for body in &mut self.bodies {
            integrate_body(body, dt, self.gravity);
        }

        self.collision_stats = self.detect_collisions();
        self.step_count += 1;
    }

    pub fn step_count(&self) -> u64 {
        self.step_count
    }

    pub fn add_body(&mut self, body: RigidBody) -> BodyHandle {
        let handle = self.bodies.len();
        self.bodies.push(body);
        handle
    }

    pub fn create_body(
        &mut self,
        body_type: BodyType,
        position: Vec2,
        collider: Option<Collider>,
    ) -> BodyHandle {
        let id = self.next_body_id;
        self.next_body_id += 1;

        self.add_body(RigidBody::new(id, body_type, position, collider))
    }

    pub fn create_dynamic_body(
        &mut self,
        position: Vec2,
        collider: Option<Collider>,
    ) -> BodyHandle {
        self.create_body(BodyType::Dynamic, position, collider)
    }

    pub fn create_static_body(&mut self, position: Vec2, collider: Option<Collider>) -> BodyHandle {
        self.create_body(BodyType::Static, position, collider)
    }

    pub fn bodies(&self) -> &[RigidBody] {
        &self.bodies
    }

    pub fn collision_stats(&self) -> CollisionStats {
        self.collision_stats
    }

    fn detect_collisions(&self) -> CollisionStats {
        let candidate_pairs = collect_collision_pairs(&self.bodies);
        let collisions = candidate_pairs
            .iter()
            .filter(|pair| {
                detect_collision(&self.bodies[pair.body_a], &self.bodies[pair.body_b]).is_some()
            })
            .count();

        CollisionStats {
            candidate_pairs: candidate_pairs.len(),
            collisions,
        }
    }

    #[cfg(test)]
    pub fn body(&self, handle: BodyHandle) -> Option<&RigidBody> {
        self.bodies.get(handle)
    }

    #[cfg(test)]
    pub fn body_mut(&mut self, handle: BodyHandle) -> Option<&mut RigidBody> {
        self.bodies.get_mut(handle)
    }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new(Vec2::new(0.0, -9.81))
    }
}

#[cfg(test)]
mod tests {
    use super::PhysicsWorld;
    use crate::{
        math::Vec2,
        physics::{
            body::{BodyType, RigidBody},
            collider::Collider,
            collision::CollisionStats,
        },
    };

    #[test]
    fn stores_and_retrieves_bodies_by_handle() {
        let mut world = PhysicsWorld::default();

        let handle = world.add_body(RigidBody {
            id: 7,
            body_type: BodyType::Dynamic,
            position: Vec2::new(1.0, 2.0),
            velocity: Vec2::new(3.0, 4.0),
            force_accumulator: Vec2::default(),
            inverse_mass: 1.0,
            restitution: 0.5,
            friction: 0.3,
            collider: None,
        });

        assert_eq!(world.bodies().len(), 1);
        assert_eq!(
            world.body(handle).map(|body| body.position),
            Some(Vec2::new(1.0, 2.0))
        );
        assert!(world.body(handle + 1).is_none());
    }

    #[test]
    fn increments_step_count() {
        let mut world = PhysicsWorld::default();
        assert_eq!(world.step_count(), 0);
        world.step(0.016);
        assert_eq!(world.step_count(), 1);
        world.step(0.016);
        assert_eq!(world.step_count(), 2);
    }

    #[test]
    fn adds_body_to_storage() {
        let mut world = PhysicsWorld::default();
        let handle = world.add_body(RigidBody {
            id: 1,
            body_type: BodyType::Dynamic,
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            force_accumulator: Vec2::default(),
            inverse_mass: 1.0,
            restitution: 0.5,
            friction: 0.5,
            collider: None,
        });

        assert_eq!(world.bodies().len(), 1);
        assert_eq!(world.body(handle).unwrap().id, 1);
    }

    #[test]
    fn creates_static_and_dynamic_bodies() {
        let mut world = PhysicsWorld::default();

        let dynamic_handle =
            world.create_dynamic_body(Vec2::new(-1.0, 3.0), Some(Collider::Circle { radius: 1.0 }));
        let static_handle = world.create_static_body(
            Vec2::new(0.0, -5.0),
            Some(Collider::Aabb {
                half_extents: (10.0, 0.5),
            }),
        );

        let dynamic_body = world.body(dynamic_handle).unwrap();
        let static_body = world.body(static_handle).unwrap();

        assert_eq!(dynamic_body.body_type, BodyType::Dynamic);
        assert_eq!(dynamic_body.inverse_mass, 1.0);
        assert_eq!(static_body.body_type, BodyType::Static);
        assert_eq!(static_body.inverse_mass, 0.0);
    }

    #[test]
    fn applies_gravity_to_dynamic_bodies() {
        let mut world = PhysicsWorld::new(Vec2::new(0.0, -10.0));
        let handle = world.create_dynamic_body(Vec2::new(0.0, 5.0), None);

        world.step(0.5);

        let body = world.body(handle).unwrap();
        assert_eq!(body.velocity, Vec2::new(0.0, -5.0));
        assert_eq!(body.position, Vec2::new(0.0, 2.5));
    }

    #[test]
    fn leaves_static_bodies_unchanged_during_step() {
        let mut world = PhysicsWorld::new(Vec2::new(0.0, -10.0));
        let handle = world.create_static_body(Vec2::new(1.0, 2.0), None);
        let body = world.body_mut(handle).unwrap();
        body.velocity = Vec2::new(3.0, -4.0);
        body.force_accumulator = Vec2::new(9.0, 1.0);

        world.step(0.25);

        let body = world.body(handle).unwrap();
        assert_eq!(body.position, Vec2::new(1.0, 2.0));
        assert_eq!(body.velocity, Vec2::default());
        assert_eq!(body.force_accumulator, Vec2::default());
    }

    #[test]
    fn applies_accumulated_force_once_per_step() {
        let mut world = PhysicsWorld::new(Vec2::default());
        let handle = world.create_dynamic_body(Vec2::new(0.0, 0.0), None);
        let body = world.body_mut(handle).unwrap();
        body.inverse_mass = 0.5;
        body.force_accumulator = Vec2::new(8.0, 0.0);

        world.step(1.0);

        let body = world.body(handle).unwrap();
        assert_eq!(body.velocity, Vec2::new(4.0, 0.0));
        assert_eq!(body.position, Vec2::new(4.0, 0.0));
        assert_eq!(body.force_accumulator, Vec2::default());
    }

    #[test]
    fn updates_collision_stats_each_step() {
        let mut world = PhysicsWorld::new(Vec2::default());
        world.create_dynamic_body(Vec2::new(0.0, 0.0), Some(Collider::Circle { radius: 1.0 }));
        world.create_dynamic_body(Vec2::new(1.5, 0.0), Some(Collider::Circle { radius: 1.0 }));
        world.create_static_body(
            Vec2::new(10.0, 0.0),
            Some(Collider::Aabb {
                half_extents: (1.0, 1.0),
            }),
        );

        world.step(0.0);

        assert_eq!(
            world.collision_stats(),
            CollisionStats {
                candidate_pairs: 3,
                collisions: 1,
            }
        );
    }
}
