// Represents the timing information for a single frame. This includes the raw frame time (the time it took to render the frame) and the clamped frame time (the raw frame time clamped to the maximum frame time).
#[derive(Clone, Copy)]
pub struct FrameSample {
    pub raw_frame_time: f32,
    pub clamped_frame_time: f32,
}

// Manages the timing state for the simulation, including the fixed time step, accumulator, pause state, and single-step requests. This is responsible for determining when to advance the simulation based on the elapsed time and user input.
pub struct TimingState {
    fixed_dt: f32,
    accumulator: f32,
    max_frame_time: f32,
    paused: bool,
    single_step_requested: bool,
    steps_this_frame: u32,
    last_frame_time: f32,
}

impl TimingState {
    pub fn new(fixed_dt: f32, max_frame_time: f32) -> Self {
        Self {
            fixed_dt,
            accumulator: 0.0,
            max_frame_time,
            paused: false,
            single_step_requested: false,
            steps_this_frame: 0,
            last_frame_time: 0.0,
        }
    }

    pub fn begin_frame(&mut self, raw_frame_time: f32) -> FrameSample {
        let clamped_frame_time = raw_frame_time.min(self.max_frame_time);
        self.accumulator += clamped_frame_time;
        self.last_frame_time = clamped_frame_time;

        FrameSample {
            raw_frame_time,
            clamped_frame_time,
        }
    }

    pub fn should_step(&mut self) -> bool {
        if self.paused {
            if self.single_step_requested {
                self.single_step_requested = false;
                return true;
            }

            return false;
        }

        if self.accumulator >= self.fixed_dt {
            self.accumulator -= self.fixed_dt;
            return true;
        }

        false
    }

    pub fn finish_frame(&mut self, steps_this_frame: u32, frame_sample: FrameSample) {
        self.steps_this_frame = steps_this_frame;
        self.last_frame_time = frame_sample.clamped_frame_time;
    }

    pub fn fixed_dt(&self) -> f32 {
        self.fixed_dt
    }

    pub fn accumulator(&self) -> f32 {
        self.accumulator
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn last_frame_time(&self) -> f32 {
        self.last_frame_time
    }

    pub fn max_frame_time(&self) -> f32 {
        self.max_frame_time
    }

    pub fn steps_this_frame(&self) -> u32 {
        self.steps_this_frame
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn request_single_step(&mut self) {
        self.single_step_requested = true;
    }
}

#[cfg(test)]
mod tests {
    use super::TimingState;

    #[test]
    fn consumes_fixed_steps_from_accumulator() {
        let mut timing = TimingState::new(1.0 / 60.0, 0.25);
        let frame = timing.begin_frame(0.04);

        let mut steps = 0;
        while timing.should_step() {
            steps += 1;
        }
        timing.finish_frame(steps, frame);

        assert_eq!(steps, 2);
        assert!(timing.accumulator() > 0.0);
        assert!(timing.accumulator() < timing.fixed_dt());
    }

    #[test]
    fn paused_mode_requires_single_step_request() {
        let mut timing = TimingState::new(1.0 / 60.0, 0.25);
        timing.toggle_pause();
        timing.begin_frame(0.1);

        assert!(!timing.should_step());

        timing.request_single_step();

        assert!(timing.should_step());
        assert!(!timing.should_step());
    }
}
