mod base;
use sqrt::sqrt;
use crate::base::factorial;

/// For Binomial Distribution
pub struct Binom{
    pub n: u32, // number of trials
    pub p: f64, // probability of success
    pub q: f64, // probability of failure
    pub x: u32, // number of success
}

impl Binom{
    pub fn mean(&self) -> f64{
        self.n as f64 * self.p
    }

    pub fn variance(&self) -> f64 {
        self.mean() * self.q
    }

    pub fn std_deviation(&self) -> f64 {
       sqrt(self.variance() as f64)
    }

    pub fn binom_distribution(&self) -> f64 {
        (factorial(self.n) as f64
        / ((factorial(self.n - self.x) as f64) * factorial(self.x) as f64))
        * ((self.p as u32 ^ self.x ) as f64 * (self.q as u32 ^ (self.n - self.x)) as f64)
    }
}

//For Uniform Distributiion
pub struct Uniform{
    pub a: u32,
    pub b: u32,
}

impl Uniform{
    pub fn mean(&self) -> f64 {
        ((self.a + self.b) / 2) as f64
    }

    pub fn fx(&self) -> f64 {
        (1 / (self.b - self.a)) as f64
    }

    pub fn std_deviation(&self) -> f64 {
        sqrt(self.b - self.a) ^ 2 / 12 as f64
    }
}