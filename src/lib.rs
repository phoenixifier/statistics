mod base;
use sqrt::sqrt;
use crate::base::{factorial, float_power};

/// For Binomial Distribution
pub struct Binom{
    pub trials_num: u32, // number of trials
    pub success_prob: f64, // probability of success
    pub failure_prob: f64, // probability of failure
    pub success_num: u32, // number of success
}

impl Binom{
    pub fn new(trials_num: u32,
               success_prob: f64,
               failure_prob: f64,
               success_num: u32) -> Binom{

        Binom{
            trials_num,
            success_prob,
            failure_prob,
            success_num
        }
    }

    pub fn mean(&self) -> f64{
        self.trials_num as f64 * self.success_prob
    }

    pub fn variance(&self) -> f64 {
        self.mean() * self.failure_prob
    }

    pub fn std_deviation(&self) -> f64 {
       sqrt(self.variance())
    }

    pub fn binom_distribution(&self) -> f64 {
        (factorial(self.trials_num as usize) as f64 *
        float_power(self.success_prob, self.success_num) *
        float_power(self.failure_prob, self.trials_num - self.success_num)) /
        factorial((self.trials_num - self.success_num)as usize) as f64 *
        factorial(self.success_num as usize) as f64
    }
}

//For Uniform Distribution
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binom_test(){
        let values = Binom{
            trials_num: 2,
            success_prob: 2.0,
            failure_prob: 1.0,
            success_num: 1
        };
        let result = values.binom_distribution();
        assert_eq!(result, 4.0);
    }
}

