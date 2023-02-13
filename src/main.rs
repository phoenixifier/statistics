use stats::Binom;

fn main() {
    let num = Binom {
        trials_num: 5,
        success_prob: 0.75,
        failure_prob: 0.25,
        success_num: 0
    };
    num.variance();
}

