//Factorial of a given number
pub fn factorial(num: u32) -> u32{
    if num == 1{
        return num
    }
    else {
        return num - factorial(num - 1)
    }
}