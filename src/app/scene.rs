use crate::math::{Aabb, Vec2};
use crate::physics::body::RigidBody;
use crate::physics::collision::CollisionStats;
use crate::physics::{collider::Collider, world::PhysicsWorld};
use crate::render::debug_draw::DebugRenderer;

// A simple scene that contains a grid, some circles, AABBs, and points for testing the debug renderer. The `DebugScene` struct holds the data for these primitives and provides a method to draw them using a `DebugRenderer`. It also has a method to count the number of each type of primitive in the scene.
#[derive(Clone, Copy)]
pub struct PrimitiveCounts {
    pub lines: usize,
    pub circles: usize,
    pub aabbs: usize,
    pub points: usize,
}

#[derive(Clone, Copy)]
pub struct CollisionSceneStats {
    pub candidate_pairs: usize,
    pub collisions: usize,
}

// The `DebugScene` struct represents a simple scene for testing the debug renderer. It contains a grid of lines, a few circles, AABBs, and points. The `draw` method uses a `DebugRenderer` to render these primitives on the screen, while the `primitive_counts` method returns the count of each type of primitive in the scene for display in the HUD.
pub struct DebugScene {
    grid_lines: Vec<(Vec2, Vec2)>,
}

impl Default for DebugScene {
    fn default() -> Self {
        let mut grid_lines = Vec::new();
        let half_width = 12;
        let half_height = 7;

        for x in -half_width..=half_width {
            grid_lines.push((
                Vec2::new(x as f32, -half_height as f32),
                Vec2::new(x as f32, half_height as f32),
            ));
        }

        for y in -half_height..=half_height {
            grid_lines.push((
                Vec2::new(-half_width as f32, y as f32),
                Vec2::new(half_width as f32, y as f32),
            ));
        }

        Self { grid_lines }
    }
}

impl DebugScene {
    pub fn initialize_world(world: &mut PhysicsWorld) {
        // Milestone 4 scene covers the supported narrow-phase pairs.
        world.create_static_body(
            Vec2::new(0.0, -5.5),
            Some(Collider::Aabb {
                half_extents: (10.0, 0.5),
            }),
        );
        world.create_dynamic_body(Vec2::new(-5.0, 1.0), Some(Collider::Circle { radius: 1.0 }));
        world.create_dynamic_body(Vec2::new(-3.4, 1.0), Some(Collider::Circle { radius: 1.0 }));
        world.create_dynamic_body(
            Vec2::new(0.0, 0.0),
            Some(Collider::Aabb {
                half_extents: (1.2, 1.2),
            }),
        );
        world.create_dynamic_body(
            Vec2::new(1.7, 0.0),
            Some(Collider::Aabb {
                half_extents: (1.0, 1.0),
            }),
        );
        world.create_static_body(
            Vec2::new(5.0, 0.0),
            Some(Collider::Aabb {
                half_extents: (1.25, 1.25),
            }),
        );
        world.create_dynamic_body(Vec2::new(6.0, 1.5), Some(Collider::Circle { radius: 1.0 }));
    }

    pub fn draw(&self, renderer: &mut DebugRenderer) {
        for (start, end) in &self.grid_lines {
            renderer.line(*start, *end);
        }
    }

    pub fn draw_world(&self, renderer: &mut DebugRenderer, world: &PhysicsWorld) {
        for body in world.bodies() {
            self.draw_body(renderer, body);
        }
    }

    pub fn primitive_counts(&self, world: &PhysicsWorld) -> PrimitiveCounts {
        let mut circles = 0;
        let mut aabbs = 0;

        for body in world.bodies() {
            match body.collider {
                Some(Collider::Circle { .. }) => circles += 1,
                Some(Collider::Aabb { .. }) => aabbs += 1,
                None => {}
            }
        }

        PrimitiveCounts {
            lines: self.grid_lines.len(),
            circles,
            aabbs,
            points: 0,
        }
    }

    pub fn collision_stats(&self, world: &PhysicsWorld) -> CollisionSceneStats {
        let CollisionStats {
            candidate_pairs,
            collisions,
        } = world.collision_stats();

        CollisionSceneStats {
            candidate_pairs,
            collisions,
        }
    }

    fn draw_body(&self, renderer: &mut DebugRenderer, body: &RigidBody) {
        match body.collider {
            Some(Collider::Circle { radius }) => renderer.circle(body.position, radius),
            Some(Collider::Aabb { half_extents }) => renderer.aabb(Aabb::from_center_half_extents(
                body.position,
                Vec2::new(half_extents.0, half_extents.1),
            )),
            None => renderer.point(body.position),
        }
    }
}
