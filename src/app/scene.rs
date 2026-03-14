use crate::math::{Aabb, Vec2};
use crate::render::debug_draw::DebugRenderer;

// A simple scene that contains a grid, some circles, AABBs, and points for testing the debug renderer. The `DebugScene` struct holds the data for these primitives and provides a method to draw them using a `DebugRenderer`. It also has a method to count the number of each type of primitive in the scene.
#[derive(Clone, Copy)]
pub struct PrimitiveCounts {
    pub lines: usize,
    pub circles: usize,
    pub aabbs: usize,
    pub points: usize,
}

// The `DebugScene` struct represents a simple scene for testing the debug renderer. It contains a grid of lines, a few circles, AABBs, and points. The `draw` method uses a `DebugRenderer` to render these primitives on the screen, while the `primitive_counts` method returns the count of each type of primitive in the scene for display in the HUD.
pub struct DebugScene {
    grid_lines: Vec<(Vec2, Vec2)>,
    circles: Vec<(Vec2, f32)>,
    aabbs: Vec<Aabb>,
    points: Vec<Vec2>,
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

        Self {
            grid_lines,
            circles: vec![(Vec2::new(-4.0, 1.5), 1.0), (Vec2::new(2.5, -1.25), 0.75)],
            aabbs: vec![
                Aabb::from_center_half_extents(Vec2::new(0.0, 0.0), Vec2::new(2.0, 1.25)),
                Aabb::from_center_half_extents(Vec2::new(5.0, 2.5), Vec2::new(1.0, 1.5)),
            ],
            points: vec![
                Vec2::new(-6.0, -3.0),
                Vec2::new(0.0, 0.0),
                Vec2::new(6.0, 3.0),
            ],
        }
    }
}

impl DebugScene {
    pub fn draw(&self, renderer: &mut DebugRenderer) {
        for (start, end) in &self.grid_lines {
            renderer.line(*start, *end);
        }

        for (center, radius) in &self.circles {
            renderer.circle(*center, *radius);
        }

        for aabb in &self.aabbs {
            renderer.aabb(*aabb);
        }

        for point in &self.points {
            renderer.point(*point);
        }
    }

    pub fn primitive_counts(&self) -> PrimitiveCounts {
        PrimitiveCounts {
            lines: self.grid_lines.len(),
            circles: self.circles.len(),
            aabbs: self.aabbs.len(),
            points: self.points.len(),
        }
    }
}
