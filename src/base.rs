//Factorial of a given number
pub fn factorial(num: usize) -> usize{
    if num == 1{
        num
    }
    else {
        num * factorial(num - 1)
    }

}

pub fn float_power(num: f64, power: u32) -> f64{
    let mut result: f64 = 1.0;

    for _i in 0..power {
        result *= num;
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_test(){
        let result = factorial(5);
        assert_eq!(result, 120);
    }

    fn power_test() {
        let result = float_power(2.4, 3);
        assert_eq!(result, 13.824);
    }
}