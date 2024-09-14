pub mod algorithms;

pub fn play<G: Guesser>(answer: &'static str, mut guesser: G) -> Option<i32> {
    let mut history = Vec::new();

    // not limiting number of guesess to get full distribution in the tail
    for i in 1..=64 { 
        let guess = guesser.guess(&history);

        if guess == answer {
            return Some(i);
        }

        let correctness = Correctness::compute(answer, &guess);
        history.push(Guess {
            mask: correctness,
            word: guess
        })
    }

    None
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Correctness {
    Correct,    // Green
    Misplaced,  // Yellow
    Wrong,      // Gray
}

impl Correctness {

    fn compute(answer: &str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);

        let mut c = [Correctness::Wrong; 5];



        return c;
    }
}

pub struct Guess {
    pub word: String,
    pub mask: [Correctness; 5],
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}