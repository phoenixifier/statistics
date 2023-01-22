use stats::Value;

fn main() {
    let num = Value {n: 5, p: 0.75, q: 0.25, x: 0};
    println!("{}", num.variance());
}

