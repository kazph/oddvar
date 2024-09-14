
const GAMES: &str = include_str!("./../resources/answers.txt");

fn main() {
    
    for answer in GAMES.split_whitespace() {
        let guesser = oddvar::algorithms::Naive::new();
        oddvar::play(answer, guesser);
    }
}
