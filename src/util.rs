use std::{
    cmp,
    time::{self, Duration},
};

#[macro_export]
macro_rules! if_else {
    ($pred:expr, $if:expr, $else:expr) => {
        if $pred {
            $if
        } else {
            $else
        }
    };
}

#[macro_export]
macro_rules! limit {
    ($op:expr, $min:expr, $max:expr) => {{
        let res = $op;
        if_else!(res < $min, $min, if_else!(res < $max, res, $max))
    }};
    ($op:expr, $max:expr) => {{
        let res = $op;
        if_else!(res < $max, res, $max)
    }};
}

#[macro_export]
macro_rules! limite {
    ($op:expr, $min:expr, $max:expr) => {{
        let res = $op;
        let max = $max.saturating_sub(1);
        if_else!(res < $min, $min, if_else!(res < $max, res, max))
    }};
    ($op:expr, $max:expr) => {{
        let res = $op;
        let max = $max.saturating_sub(1);
        if_else!(res < $max, res, max)
    }};
}

#[macro_export]
macro_rules! err_at {
    ($e:expr) => {{
        use Error::{BadPattern, Fatal, IOError, Invalid, NoTopic, IPC};
        use Error::{FailBuffer, FailConvert, FailParse};

        let p = format!("{}:{}", file!(), line!());
        match $e {
            Ok(val) => Ok(val),
            Err(Fatal(_, s)) => Err(Fatal(p, s)),
            Err(BadPattern(_, s)) => Err(BadPattern(p, s)),
            Err(IOError(_, s)) => Err(IOError(p, s)),
            Err(IPC(_, s)) => Err(IPC(p, s)),
            Err(NoTopic(_)) => Err(NoTopic(p)),
            Err(Invalid(_, s)) => Err(Invalid(p, s)),
            Err(FailConvert(_, s)) => Err(FailConvert(p, s)),
            Err(FailParse(_, s)) => Err(FailParse(p, s)),
            Err(FailBuffer(_, s)) => Err(FailBuffer(p, s)),
        }
    }};
    ($v:ident, msg:$m:expr) => {{
        let prefix = format!("{}:{}", file!(), line!());
        Err(Error::$v(prefix, format!("{}", $m)))
    }};
    ($v:ident, $e:expr) => {
        match $e {
            Ok(val) => Ok(val),
            Err(err) => {
                let prefix = format!("{}:{}", file!(), line!());
                Err(Error::$v(prefix, format!("{}", err)))
            }
        }
    };
    ($v:ident, $e:expr, $m:expr) => {
        match $e {
            Ok(val) => Ok(val),
            Err(err) => {
                let prefix = format!("{}:{}", file!(), line!());
                Err(Error::$v(prefix, format!("{} {}", $m, err)))
            }
        }
    };
}

pub fn time_it<F, R>(stats: &mut Latency, callb: F) -> R
where
    F: FnOnce() -> R,
{
    let start = time::Instant::now();
    let res = callb();
    stats.sample(start.elapsed());
    res
}

#[derive(Clone, Default, Debug)]
pub struct Latency {
    name: String,
    samples: usize,
    min: Duration,
    max: Duration,
    total: Duration,
    durations: Vec<usize>,
}

impl Latency {
    pub fn new(name: &str) -> Latency {
        let mut stats: Latency = Latency::default();
        stats.name = name.to_string();
        stats.durations = Vec::with_capacity(256);
        stats.durations.resize(256, 0);
        stats
    }

    pub fn sample(&mut self, duration: Duration) {
        self.samples += 1;
        self.total += duration;
        if self.min == Duration::from_nanos(0) || self.min > duration {
            self.min = duration
        }
        if self.max == Duration::from_nanos(0) || self.max < duration {
            self.max = duration
        }
        let off = {
            let off = (duration.as_nanos() / 1_000_000) as usize;
            cmp::min(off, 255)
        };
        self.durations[off] += 1;
    }

    #[allow(dead_code)]
    fn samples(&self) -> usize {
        self.samples
    }

    #[allow(dead_code)]
    fn min(&self) -> Duration {
        self.min
    }

    #[allow(dead_code)]
    fn max(&self) -> Duration {
        self.max
    }

    fn mean(&self) -> Duration {
        if self.samples > 0 {
            self.total / (self.samples as u32)
        } else {
            Duration::from_secs(0)
        }
    }

    fn percentiles(&self) -> Vec<(u8, usize)> {
        let mut percentiles: Vec<(u8, usize)> = vec![];
        let (mut acc, mut prev_perc) = (0_f64, 90_u8);
        let iter = self
            .durations
            .iter()
            .enumerate()
            .filter(|(_, &item)| item > 0);
        for (duration, samples) in iter {
            acc += *samples as f64;
            let perc = ((acc / (self.samples as f64)) * 100_f64) as u8;
            if perc >= prev_perc {
                percentiles.push((perc, duration));
                prev_perc = perc;
            }
        }
        percentiles
    }

    pub fn pretty_print(&self) -> String {
        let mean = self.mean();
        let mut outs = format!(
            "{:?} duration min:{:?}, avg:{:?}, max:{:?}",
            self.name, self.min, mean, self.max
        );
        for (dur, n) in self.percentiles().into_iter() {
            if n > 0 {
                outs.push_str(&format!("  {}-percentile = {}", dur, n));
            }
        }

        outs
    }
}
