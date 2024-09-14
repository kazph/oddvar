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
        
        // Mark things as correct
        for (i, (a, g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                c[i] = Correctness::Correct;
            }
        }

        // mark things as misplaced
        let mut marked = [false; 5];
        for (i, &c) in c.iter().enumerate() {
            if c == Correctness::Correct {
                marked[i] = true;
            }
        }

        for (i, g) in guess.chars().enumerate() {
            if c[i] == Correctness::Correct {
                continue;
            }

            if answer.chars().enumerate().any(|(i, a)| {
                if a == g && !marked[i] {
                    marked[i] = true;
                    return true
                }
                return false;
            }) {
                c[i] = Correctness::Misplaced;
            }
        }

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

#[cfg(test)]
macro_rules! mask {
    (C) => {$crate::Correctness::Correct};
    (M) => {$crate::Correctness::Misplaced};
    (W) => {$crate::Correctness::Wrong};
    ($($c:tt)+) => {[
        $(mask!($c)),+
    ]}
}

#[cfg(test)]
mod tests {
    use crate::Correctness;

    macro_rules! check {
        (C) => {Correctness::Correct};
        (M) => {Correctness::Misplaced};
        (W) => {Correctness::Wrong};
        ([$(tt)+]) => [

        ]
    }

    #[test]
    fn compute() {
        assert_eq!(Correctness::compute("abcde", "abcde"), mask![C C C C C]);
        assert_eq!(Correctness::compute("abcde", "bcdea"), mask![M M M M M]);
        assert_eq!(Correctness::compute("abcde", "fghij"), mask![W W W W W]);
        assert_eq!(Correctness::compute("aabcd", "baddd"), mask![M C W W C]);
        assert_eq!(Correctness::compute("azzaz", "aaabb"), mask![C M W W W]);
        assert_eq!(Correctness::compute("kasph", "simba"), mask![M W W W M]);
        assert_eq!(Correctness::compute("baccc", "aaddd"), mask![W C W W W]);
    }
}