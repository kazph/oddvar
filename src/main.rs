
const GAMES: &str = include_str!("./../resources/answers.txt");

fn main() {
    let guesser = oddvar::algorithms::Naive::new();
    
    for answer in GAMES.split_whitespace() {
        oddvar::play(answer, guesser)
    }
}
