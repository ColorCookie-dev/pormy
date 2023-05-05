enum Status {
    Running{
        start_time: std::time::Instant
    },
    Paused,
}

pub struct Stopwatch {
    time_elapsed: std::time::Duration,
    status: Status,
}

impl Stopwatch {
    pub fn start() -> Self {
        Self {
            time_elapsed: std::time::Duration::from_secs(0),
            status: Status::Running { start_time: std::time::Instant::now() }
        }
    }

    pub fn pause(&mut self) {
        if let Status::Running { start_time } = self.status {
            self.time_elapsed += std::time::Instant::now()
                .saturating_duration_since(start_time);
            self.status = Status::Paused;
        }
    }

    pub fn resume(&mut self) {
        if let Status::Paused = self.status {
            self.status = Status::Running {
                start_time: std::time::Instant::now()
            };
        }
    }

    pub fn toggle(&mut self) {
        if self.is_running() {
            self.pause();
        } else {
            self.resume();
        }
    }

    pub fn is_running(&self) -> bool {
        match self.status {
            Status::Running { start_time: _ } => true,
            Status::Paused => false,
        }
    }

    pub fn time_elapsed(&self) -> std::time::Duration {
        match self.status {
            Status::Running { start_time } => self.time_elapsed.clone() +
                std::time::Instant::now().saturating_duration_since(start_time),
            Status::Paused => self.time_elapsed.clone(),
        }
    }
}

