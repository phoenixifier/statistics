//Factorial of a given number
pub fn factorial(num: u32) -> u32{
    if num == 1{
        return num
    }
    else {
        return num * factorial(num - 1)
    }
}

//pub fn powi(self, n: i32) -> f64

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_test(){
        let result = factorial(5);
        assert_eq!(result, 120);
    }
}