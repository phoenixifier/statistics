mod base;
use sqrt::sqrt;
use crate::base::factorial;

/// For Binomial Distribution
pub struct Binom{
    pub trials_num: u32, // number of trials
    pub success_probability: f64, // probability of success
    pub failure_probability: f64, // probability of failure
    pub successes_num: u32, // number of success
}

impl Binom{
    pub fn new(trials_num: u32,
               success_probability: f64,
               failure_probability: f64,
               successes_num: u32) -> Binom{
        Binom{
            trials_num,
            success_probability,
            failure_probability,
            successes_num
        }
    }

    pub fn mean(&self) -> f64{
        self.trials_num as f64 * self.success_probability
    }

    pub fn variance(&self) -> f64 {
        self.mean() * self.failure_probability
    }

    pub fn std_deviation(&self) -> f64 {
       sqrt(self.variance() as f64)
    }

    pub fn binom_distribution(&self) -> f64 {
        (factorial(self.trials_num) as f64 /
        ((factorial(self.trials_num - self.successes_num) as f64) *
            factorial(self.successes_num) as f64)) *
        ((self.success_probability as u32 ^ self.successes_num ) as f64 *
            (self.failure_probability as u32 ^ (self.trials_num - self.successes_num)) as f64)
    }
}

//For Uniform Distributiion
pub struct Uniform{
    pub a: f64,
    pub b: f64,
}

impl Uniform{
    pub fn new(a: f64, b: f64) -> Uniform{
        Uniform{
            a,
            b
        }
    }
    pub fn mean(&self) -> f64 {
        (self.a + self.b) / 2 as f64
    }

    pub fn fx(&self) -> f64 {
        1 as f64 / (self.b - self.a)
    }

    pub fn std_deviation(&self) -> f64 {
        let square_root = sqrt(self.b - self.a);
        float_power(square_root , 2 ) / 12 as f64
    }
}

fn float_power(num: f64, power: u32) -> f64{
    let mut result: f64 = 1.0;

    for i in 0..power {
        result *= num;
    }
    result
}

#[cfg(test)]
mod tests{
   use super::*;

    #[test]
    fn power_test() {
        assert_eq!(float_power(2.4, 3), 13.824);
    }
}