use macroquad::prelude::{Color, color_u8, draw_text, measure_text};

use crate::{
    app::{CollisionSceneStats, PrimitiveCounts, TimingState},
    math::Vec2,
    physics::{
        body::{BodyType, RigidBody},
        collider::Collider,
        world::PhysicsWorld,
    },
};

#[derive(Clone, Copy)]
pub struct BodyReadout {
    pub id: u32,
    pub body_kind: &'static str,
    pub collider_kind: &'static str,
    pub position: Vec2,
    pub velocity: Vec2,
    pub force: Vec2,
}

impl BodyReadout {
    fn from_body(body: &RigidBody) -> Self {
        Self {
            id: body.id,
            body_kind: match body.body_type {
                BodyType::Dynamic => "dyn",
                BodyType::Static => "sta",
            },
            collider_kind: match body.collider {
                Some(Collider::Circle { .. }) => "circle",
                Some(Collider::Aabb { .. }) => "aabb",
                None => "point",
            },
            position: body.position,
            velocity: body.velocity,
            force: body.force_accumulator,
        }
    }
}

pub fn body_readouts(world: &PhysicsWorld) -> Vec<BodyReadout> {
    world.bodies().iter().map(BodyReadout::from_body).collect()
}

// Responsible for rendering the HUD (heads-up display) with timing information and primitive counts.
pub struct HudRenderer {
    left_margin: f32,
    line_height: f32,
    top_margin: f32,
}

impl Default for HudRenderer {
    fn default() -> Self {
        Self {
            left_margin: 20.0,
            line_height: 20.0,
            top_margin: 28.0,
        }
    }
}

impl HudRenderer {
    pub fn draw(
        &self,
        timing: &TimingState,
        raw_frame_time: f32,
        step_count: u64,
        primitive_counts: PrimitiveCounts,
        collision_stats: CollisionSceneStats,
        body_readouts: &[BodyReadout],
    ) {
        let fps = if raw_frame_time > 0.0 {
            1.0 / raw_frame_time
        } else {
            0.0
        };
        let run_state = if timing.is_paused() {
            "Space pause | N single-step | Esc quit | paused".to_owned()
        } else {
            "Space pause | N single-step | Esc quit | running".to_owned()
        };

        let hud_lines = [
            "M4 collision detection".to_owned(),
            run_state,
            format!("fps: {:.1}", fps),
            format!("frame dt: {:.2} ms", raw_frame_time * 1000.0),
            format!("clamped dt: {:.2} ms", timing.last_frame_time() * 1000.0),
            format!("fixed dt: {:.2} ms", timing.fixed_dt() * 1000.0),
            format!("accumulator: {:.3} s", timing.accumulator()),
            format!("steps this frame: {}", timing.steps_this_frame()),
            format!("sim steps total: {}", step_count),
            format!(
                "primitives: {} lines, {} circles, {} aabbs, {} points",
                primitive_counts.lines,
                primitive_counts.circles,
                primitive_counts.aabbs,
                primitive_counts.points
            ),
            format!(
                "collision pairs: {} candidates, {} overlaps",
                collision_stats.candidate_pairs, collision_stats.collisions
            ),
        ];

        let mut y = self.top_margin;
        for line in &hud_lines {
            draw_text(
                line,
                self.left_margin,
                y,
                24.0,
                color_u8!(233, 236, 240, 255),
            );
            y += self.line_height;
        }

        y += self.line_height;
        draw_text(
            "bodies:",
            self.left_margin,
            y,
            24.0,
            color_u8!(200, 208, 218, 255),
        );
        y += self.line_height;

        for readout in body_readouts {
            let line = format!(
                "body {} | {} {} | p=({:.2}, {:.2}) | v=({:.2}, {:.2}) | f=({:.2}, {:.2})",
                readout.id,
                readout.body_kind,
                readout.collider_kind,
                readout.position.x,
                readout.position.y,
                readout.velocity.x,
                readout.velocity.y,
                readout.force.x,
                readout.force.y,
            );

            draw_text(
                &line,
                self.left_margin,
                y,
                22.0,
                color_u8!(233, 236, 240, 255),
            );
            y += self.line_height;
        }

        let scale_label = format!(
            "max frame clamp: {:.0} ms",
            timing.max_frame_time() * 1000.0
        );
        let label_metrics = measure_text(&scale_label, None, 22, 1.0);
        draw_text(
            &scale_label,
            macroquad::prelude::screen_width() - label_metrics.width - self.left_margin,
            self.top_margin,
            22.0,
            color_u8!(158, 168, 184, 255),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::body_readouts;
    use crate::{
        math::Vec2,
        physics::{collider::Collider, world::PhysicsWorld},
    };

    #[test]
    fn maps_world_bodies_to_hud_readouts() {
        let mut world = PhysicsWorld::default();
        let floor = world.create_static_body(
            Vec2::new(0.0, -4.5),
            Some(Collider::Aabb {
                half_extents: (10.0, 0.5),
            }),
        );
        let falling =
            world.create_dynamic_body(Vec2::new(0.0, 3.0), Some(Collider::Circle { radius: 0.75 }));
        let body = world.body_mut(falling).unwrap();
        body.velocity = Vec2::new(1.5, -2.0);
        body.force_accumulator = Vec2::new(0.0, -9.81);

        let readouts = body_readouts(&world);

        assert_eq!(readouts.len(), 2);

        let floor_readout = readouts[floor];
        assert_eq!(floor_readout.id, 0);
        assert_eq!(floor_readout.body_kind, "sta");
        assert_eq!(floor_readout.collider_kind, "aabb");
        assert_eq!(floor_readout.position, Vec2::new(0.0, -4.5));
        assert_eq!(floor_readout.velocity, Vec2::default());

        let falling_readout = readouts[falling];
        assert_eq!(falling_readout.id, 1);
        assert_eq!(falling_readout.body_kind, "dyn");
        assert_eq!(falling_readout.collider_kind, "circle");
        assert_eq!(falling_readout.position, Vec2::new(0.0, 3.0));
        assert_eq!(falling_readout.velocity, Vec2::new(1.5, -2.0));
        assert_eq!(falling_readout.force, Vec2::new(0.0, -9.81));
    }
}
