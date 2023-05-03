use std::time::{Duration, Instant};

pub enum Status {
    Running,
    Paused,
}

impl Status {
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running)
    }

    pub fn is_paused(&self) -> bool {
        matches!(self, Self::Running)
    }
}

impl Default for Status {
    fn default() -> Self {
        Self::Running
    }
}

pub struct TimeCount {
    started: Instant,
    elapsed: Duration,
    status: Status,
}

impl Default for TimeCount {
    fn default() -> Self {
        Self {
            started: Instant::now(),
            elapsed: Duration::default(),
            status: Status::default(),
        }
    }
}

impl TimeCount {
    pub fn is_running(&self) -> bool {
        self.status.is_running()
    }

    pub fn is_paused(&self) -> bool {
        self.status.is_paused()
    }

    pub fn toggle(&mut self) {
        match self.status {
            Status::Running => {
                self.status = Status::Paused;
                self.elapsed += self.started.elapsed();
            }
            Status::Paused => {
                self.status = Status::Running;
                self.started = Instant::now();
            }
        }
    }

    pub fn elapsed(&self) -> Duration {
        match self.status {
            Status::Running => self.elapsed + self.started.elapsed(),
            Status::Paused => self.elapsed,
        }
    }
}

type Stopwatch = TimeCount;

pub struct Timer {
    counter: TimeCount,
    target: Duration,
}

impl Timer {
    pub fn new(target: Duration) -> Self {
        Self {
            counter: TimeCount::default(),
            target,
        }
    }

    pub fn has_ended(&self) -> bool {
        self.counter.elapsed() > self.target
    }

    pub fn time_left(&self) -> Option<Duration> {
        self.target.checked_sub(self.counter.elapsed())
    }

    pub fn time_exceeded(&self) -> Duration {
        self.counter.elapsed().saturating_sub(self.target)
    }

    pub fn toggle(&mut self) {
        self.counter.toggle();
    }
}

pub enum Mode {
    Work,
    Rest,
    LongRest,
}

pub struct PomoSettings {
    work: Duration,
    rest: Duration,
    long_rest: Duration,
}

impl Default for PomoSettings {
    fn default() -> Self {
        Self {
            work: Duration::from_secs(25 * 60),
            rest: Duration::from_secs(5 * 60),
            long_rest: Duration::from_secs(10 * 60),
        }
    }
}

impl PomoSettings {
    pub fn new(work: Duration, rest: Duration, long_rest: Duration) -> Self {
        Self {
            work,
            rest,
            long_rest,
        }
    }
}

pub struct Pomodoro {
    counter: Timer,
    session: u64,
    mode: Mode,
    options: PomoSettings,
}

impl Default for Pomodoro {
    fn default() -> Self {
        let def_opts = PomoSettings::default();
        Self {
            counter: Timer::new(def_opts.work),
            session: 0,
            mode: Mode::Work,
            options: def_opts,
        }
    }
}

impl Pomodoro {
    pub fn new(work: Duration, rest: Duration, long_rest: Duration) -> Self {
        let options = PomoSettings::new(work, rest, long_rest);

        Self {
            counter: Timer::new(options.work),
            session: 0,
            mode: Mode::Work,
            options,
        }
    }

    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn has_ended(&self) -> bool {
        self.counter.has_ended()
    }

    pub fn next_mode(&mut self) {
        match self.mode {
            Mode::Work => {
                if self.session % 4 == 0 {
                    self.mode = Mode::LongRest;
                    self.counter = Timer::new(self.options.long_rest);
                } else {
                    self.mode = Mode::Rest;
                    self.counter = Timer::new(self.options.rest);
                }
            }
            Mode::Rest | Mode::LongRest => {
                self.mode = Mode::Work;
                self.counter = Timer::new(self.options.work);
                self.session += 1;
            }
        }
    }

    pub fn time_left(&self) -> Option<Duration> {
        self.counter.time_left()
    }

    pub fn time_exceeded(&self) -> Duration {
        self.counter.time_exceeded()
    }

    pub fn toggle(&mut self) {
        self.counter.toggle();
    }
}
