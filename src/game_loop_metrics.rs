use std::time::{Duration, SystemTime};

struct FpsCounter {
    acc: f64,
    acc_counter: usize,
    fps_refresh_rate: f64,
    frame_count: u128,
}

impl FpsCounter {
    fn new(fps_refresh_rate: Duration) -> FpsCounter {
        FpsCounter {
            acc: 0.0,
            acc_counter: 0,
            fps_refresh_rate: fps_refresh_rate.as_secs_f64(),
            frame_count: 0,
        }
    }

    fn update(&mut self, dt: Duration) -> Option<usize> {
        self.frame_count += 1;
        self.acc += dt.as_secs_f64();
        self.acc_counter += 1;
        if self.acc >= self.fps_refresh_rate {
            let fps = (1.0 / (self.acc / self.acc_counter as f64)) as usize;
            self.acc -= self.fps_refresh_rate;
            self.acc_counter = 0;
            Some(fps)
        } else {
            None
        }
    }
}

pub struct GameLoopMetrics {
    fps_counter: FpsCounter,
    last: SystemTime,
    fps_cache: usize,
}

impl GameLoopMetrics {
    pub fn new(fps_refresh_rate: Duration) -> GameLoopMetrics {
        GameLoopMetrics {
            fps_counter: FpsCounter::new(fps_refresh_rate),
            last: SystemTime::now(),
            fps_cache: 0,
        }
    }

    pub fn update(&mut self) -> anyhow::Result<Duration> {
        let dt = self.last.elapsed()?;
        self.last = SystemTime::now();
        if let Some(fps) = self.fps_counter.update(dt) {
            self.fps_cache = fps;
        }
        Ok(dt)
    }

    pub fn fps(&self) -> usize {
        self.fps_cache
    }
}