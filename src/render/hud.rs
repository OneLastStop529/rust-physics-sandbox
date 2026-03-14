use macroquad::prelude::{Color, color_u8, draw_text, measure_text};

use crate::app::{PrimitiveCounts, TimingState};

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
            "M1 shell".to_owned(),
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
