use stats::Binom;

fn main() {
    let num = Binom {
        trials_num: 5,
        success_probability: 0.75,
        failure_probability: 0.25,
        successes_num: 0
    };
    num.variance();
}

