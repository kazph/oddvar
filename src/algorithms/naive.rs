use once_cell::sync::OnceCell;
use std::borrow::Cow;

use crate::{Correctness, Guess, Guesser, Word, DICTIONARY};

static INITIAL: OnceCell<Vec<(&'static Word, usize)>> = OnceCell::new();
static PATTERNS: OnceCell<Vec<[Correctness; 5]>> = OnceCell::new();

pub struct Naive {
    remaining: Cow<'static, Vec<(&'static Word, usize)>>,
    patterns: Cow<'static, Vec<[Correctness; 5]>>,
}

impl Naive {
    pub fn new() -> Self {
        Naive {
            remaining: Cow::Borrowed(INITIAL.get_or_init(|| {
                let mut words = Vec::from_iter(DICTIONARY.lines().map(|line| {
                    let (word, count) = line.split_once(" ").expect(
                        "All lines should have a space between word and number of occurnces!",
                    );

                    let count: usize = count.parse().expect("Count is a number");

                    return (
                        word.as_bytes()
                            .try_into()
                            .expect("Every dict word is five characters!"),
                        count,
                    );
                }));

                words.sort_unstable_by_key(|&(_, count)| std::cmp::Reverse(count));
                return words;
            })),
            patterns: Cow::Borrowed(PATTERNS.get_or_init(|| Correctness::patterns().collect())),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Candidate {
    word: &'static Word,
    goodness: f64,
}

impl Guesser for Naive {
    // The implmentation of the algorithm
    fn guess(&mut self, history: &[Guess]) -> Word {
        if let Some(last) = history.last() {
            if matches!(self.remaining, Cow::Owned(_)) {
                self.remaining
                    .to_mut()
                    .retain(|(word, _)| last.matches(word));
            } else {
                self.remaining = Cow::Owned(
                    self.remaining
                        .iter()
                        .filter(|(word, _)| last.matches(word))
                        .copied()
                        .collect(),
                )
            }
        }

        if history.is_empty() {
            self.patterns = Cow::Borrowed(PATTERNS.get().unwrap());
            return *b"tares";
        } else {
            assert!(!self.patterns.is_empty())
        }

        let remaining_count: usize = self.remaining.iter().map(|&(_, c)| c).sum();

        let mut best: Option<Candidate> = None;
        let n = std::cmp::max(std::cmp::min(self.remaining.len() / 2, 128), 32);

        for &(word, count) in self.remaining.iter().take(n) {
            let mut sum = 0.0;
            let check_pattern = |pattern: &[Correctness; 5]| {
                // Goodness = -sum_i p_i * log_2(p_i)
                // If we guessed word (outer loop) and got pattern (inner loop);
                // what is the uncertainty left?

                let mut in_pattern_total = 0;

                for (candidate, count) in &*self.remaining {
                    let g = Guess {
                        word: Cow::Borrowed(word),
                        mask: *pattern,
                    };

                    if g.matches(candidate) {
                        in_pattern_total += count;
                    }
                }

                if in_pattern_total == 0 {
                    return false;
                }

                // TODO: Sigmoid
                let p_of_pattern = in_pattern_total as f64 / remaining_count as f64;
                sum += p_of_pattern * p_of_pattern.log2();

                return true;
            };

            if matches!(self.patterns, Cow::Owned(_)) {
                self.patterns.to_mut().retain(check_pattern);
            } else {
                self.patterns = Cow::Owned(
                    self.patterns
                        .iter()
                        .copied()
                        .filter(check_pattern)
                        .collect(),
                )
            }

            let p_word = count as f64 / remaining_count as f64;
            let goodness = p_word * (0.0 - sum);

            if let Some(c) = best {
                if goodness > c.goodness {
                    best = Some(Candidate { word, goodness })
                }
            } else {
                best = Some(Candidate { word, goodness })
            }
        }

        return *best.unwrap().word;
    }
}
