const answers: &str = include_str!("../answers.txt");

fn main() {
    for answer in answers.split_whitespace() {
        let guesser = dlewor::algos::Naive::new();
        dlewor::play(answer, guesser);
    }
}
