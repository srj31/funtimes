pub mod algos;
pub fn play<G: Guesser>(answer: &'static str, mut guesser: G) -> usize {
    let mut history: Vec<Guess> = Vec::new();
    for i in 1.. {
        let guess = guesser.guess(&history[..]);
        if guess == answer {
            return i;
        }
        let correct = Check::check(answer, &guess);
        history.push(Guess {
            word: guess,
            mask: correct,
        })
    }
    panic!("Its alr usize::max")
}

pub enum Check {
    Correct,
    Present,
    Wrong,
}

impl Check {
    fn check(answer: &str, guess: &str) -> [Self; 5] {
        todo!()
    }
}

pub struct Guess {
    pub word: String,
    pub mask: [Check; 5],
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}
