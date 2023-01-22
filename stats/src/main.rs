use stats::Binom;

fn main() {
    let num = Binom {n: 5, p: 0.75, q: 0.25, x: 0};
    println!("{}", num.variance());
}

