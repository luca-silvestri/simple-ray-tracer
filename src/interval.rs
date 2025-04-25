pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn empty() -> Self {
        Interval {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    pub fn universe() -> Self {
        Interval {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }

    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn from_intervals(a: &Interval, b: &Interval) -> Self {
        Interval {
            min: f64::min(a.min, b.min),
            max: f64::max(a.max, b.max),
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && self.max >= x
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && self.max > x
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        return x;
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }
}
