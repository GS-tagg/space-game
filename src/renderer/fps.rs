use std::thread::sleep;
use std::time::{Duration, Instant};

pub struct FpsTracker {
    frame_duration: Duration,
    fps_counter: u32,
    fps_timer: Instant,
    next_frame_time: Instant,
    render_time_accum: Duration,
    last_frame_render_time: Duration,
}

impl FpsTracker {
    pub fn new(target_fps: f32) -> Self {
        let frame_duration = Duration::from_secs_f32(1.0 / target_fps);
        let now = Instant::now();

        Self {
            frame_duration,
            fps_counter: 0,
            fps_timer: now,
            next_frame_time: now + frame_duration,
            render_time_accum: Duration::ZERO,
            last_frame_render_time: Duration::ZERO,
        }
    }

    pub fn begin_render(&self) -> Instant {
        Instant::now()
    }

    pub fn end_render(&mut self, start: Instant) {
        let render_time = start.elapsed();
        self.last_frame_render_time = render_time;
        self.render_time_accum += render_time;
    }

    pub fn tick(&mut self) {
        let now = Instant::now();

        if now < self.next_frame_time {
            sleep(self.next_frame_time - now);
        }

        self.next_frame_time += self.frame_duration;
        self.fps_counter += 1;

        if self.fps_timer.elapsed() >= Duration::from_secs(1) {
            println!("Actual FPS: {}", self.fps_counter);
            println!(
                "Last Frame Render Time: {:.2} ms",
                self.last_frame_render_time.as_secs_f32() * 1000.0
            );
            println!(
                "Average Render Time: {:.2} ms",
                self.render_time_accum.as_secs_f32() * 1000.0 / self.fps_counter as f32
            );
            self.fps_counter = 0;
            self.fps_timer = Instant::now();
            self.render_time_accum = Duration::ZERO;
        }
    }
}
