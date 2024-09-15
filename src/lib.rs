use std::{borrow::Cow, collections::HashSet};

pub mod algorithms;

pub const DICTIONARY: &str = include_str!("./../resources/dictionary.txt");

pub type Word = [u8; 5];

pub struct Wordle {
    dictonary: HashSet<&'static Word>, // Known at compile time, TOOD: could use perfect hash with build script!
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            dictonary: HashSet::from_iter(DICTIONARY.lines().map(|line| {
                line.split_once(" ")
                    .expect("Every line should be word and occurences seperated by ' '")
                    .0
                    .as_bytes()
                    .try_into()
                    .expect("Every word should be 5 characters!")
            })),
        }
    }

    pub fn play<G: Guesser>(&self, answer: Word, mut guesser: G) -> Option<i32> {
        let mut history = Vec::new();

        // not limiting number of guesess to get full distribution in the tail
        for i in 1..=64 {
            let guess = guesser.guess(&history);

            if guess == answer {
                return Some(i);
            }

            /*debug_*/
            assert!(
                self.dictonary.contains(&guess),
                "guess '{}' is not in the dictonary!",
                std::str::from_utf8(&guess).unwrap()
            );

            let correctness = Correctness::compute(&answer, &guess);
            history.push(Guess {
                mask: correctness,
                word: Cow::Owned(guess),
            })
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Correctness {
    Correct,   // Green
    Misplaced, // Yellow
    Wrong,     // Gray
}

impl Correctness {
    fn compute(answer: &Word, guess: &Word) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);

        let mut c = [Correctness::Wrong; 5];

        // Mark things as correct
        for (i, (a, g)) in answer.iter().zip(guess.iter()).enumerate() {
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

        for (i, g) in guess.iter().enumerate() {
            if c[i] == Correctness::Correct {
                continue;
            }

            if answer.iter().zip(marked.iter_mut()).any(|(a, used)| {
                if a == g && !*used {
                    *used = true;
                    return true;
                }
                return false;
            }) {
                c[i] = Correctness::Misplaced;
            }
        }

        return c;
    }

    pub fn patterns() -> impl Iterator<Item = [Self; 5]> {
        itertools::iproduct!(
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong]
        )
        .map(|(a, b, c, d, e)| [a, b, c, d, e])
    }
}

pub struct Guess<'a> {
    pub word: Cow<'a, Word>,
    pub mask: [Correctness; 5],
}

impl Guess<'_> {
    pub fn matches(&self, word: &Word) -> bool {
        Correctness::compute(word, &self.word) == self.mask
    }
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> Word;
}

// For testing purposes
impl Guesser for fn(history: &[Guess]) -> Word {
    fn guess(&mut self, history: &[Guess]) -> Word {
        (*self)(history)
    }
}

#[cfg(test)]
macro_rules! guesser {
    (|$history:ident| $impl:block) => {{
        struct G;
        impl $crate::Guesser for G {
            fn guess(&mut self, $history: &[Guess]) -> $crate::Word {
                $impl
            }
        }
        G
    }};
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
    use crate::{Correctness, Guess, Wordle};

    macro_rules! check {
        (C) => {
            Correctness::Correct
        };
        (M) => {
            Correctness::Misplaced
        };
        (W) => {
            Correctness::Wrong
        };
        ([$(tt)+]) => {};
    }

    #[test]
    fn compute() {
        assert_eq!(Correctness::compute(b"abcde", b"abcde"), mask![C C C C C]);
        assert_eq!(Correctness::compute(b"abcde", b"bcdea"), mask![M M M M M]);
        assert_eq!(Correctness::compute(b"abcde", b"fghij"), mask![W W W W W]);
        assert_eq!(Correctness::compute(b"aabcd", b"baddd"), mask![M C W W C]);
        assert_eq!(Correctness::compute(b"azzaz", b"aaabb"), mask![C M W W W]);
        assert_eq!(Correctness::compute(b"kasph", b"simba"), mask![M W W W M]);
        assert_eq!(Correctness::compute(b"baccc", b"aaddd"), mask![W C W W W]);
    }

    #[test]
    fn play() {
        let w = Wordle::new();
        let guesser = guesser!(|_history| { *b"right" });
        assert_eq!(w.play(*b"right", guesser), Some(1));

        let guesser = guesser!(|_history| { *b"right" });
        assert_eq!(w.play(*b"wrong", guesser), None);
    }
}
