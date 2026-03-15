use macroquad::prelude::*;

use crate::app::scene::DebugScene;
use crate::app::timing::{FrameSample, TimingState};
use crate::input::controls::ControlState;
use crate::physics::world::PhysicsWorld;
use crate::render::debug_draw::{DebugCamera, DebugRenderer};
use crate::render::hud::HudRenderer;

// The main application struct that manages the game loop, input, physics world, and rendering.
pub struct App {
    controls: ControlState,
    scene: DebugScene,
    timing: TimingState,
    world: PhysicsWorld,
}

impl App {
    pub fn new() -> Self {
        let mut world = PhysicsWorld::default();
        DebugScene::initialize_world(&mut world);

        Self {
            controls: ControlState::default(),
            scene: DebugScene::default(),
            timing: TimingState::new(1.0 / 60.0, 0.25),
            world,
        }
    }

    pub async fn run(&mut self) {
        loop {
            self.controls = ControlState::poll();

            if self.controls.quit_requested {
                break;
            }

            let frame_sample = self.timing.begin_frame(get_frame_time());
            self.update(frame_sample);
            self.render(frame_sample);

            next_frame().await;
        }
    }

    fn update(&mut self, frame_sample: FrameSample) {
        let mut steps_this_frame = 0_u32;

        if self.controls.pause_toggled {
            self.timing.toggle_pause();
        }

        if self.controls.single_step_requested {
            self.timing.request_single_step();
        }

        while self.timing.should_step() {
            self.world.step(self.timing.fixed_dt());
            steps_this_frame += 1;
        }

        self.timing.finish_frame(steps_this_frame, frame_sample);
    }

    fn render(&self, frame_sample: FrameSample) {
        clear_background(color_u8!(14, 18, 24, 255));

        let debug_camera = DebugCamera::from_screen(screen_width(), screen_height());
        let mut debug_renderer = DebugRenderer::new(debug_camera);

        self.scene.draw(&mut debug_renderer);
        self.scene.draw_world(&mut debug_renderer, &self.world);

        let hud_renderer = HudRenderer::default();
        hud_renderer.draw(
            &self.timing,
            frame_sample.raw_frame_time,
            self.world.step_count(),
            self.scene.primitive_counts(&self.world),
        );
    }
}
