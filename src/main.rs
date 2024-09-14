
const GAMES: &str = include_str!("./../resources/answers.txt");

fn main() {
    
    let w = oddvar::Wordle::new();

    for answer in GAMES.split_whitespace() {
        let guesser = oddvar::algorithms::Naive::new();
        w.play(answer, guesser);
    }
}
