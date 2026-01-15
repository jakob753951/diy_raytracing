#[derive(Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

const universe: Interval = Interval {
    min: f64::NEG_INFINITY,
    max: f64::INFINITY,
};

const empty: Interval = Interval {
    min: f64::INFINITY,
    max: f64::NEG_INFINITY,
};

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}
