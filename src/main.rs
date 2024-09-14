
const GAMES: &str = include_str!("./../resources/answers.txt");

fn main() {
    
    let w = oddvar::Wordle::new();

    for answer in GAMES.split_whitespace() {
        let guesser = oddvar::algorithms::Naive::new();
        if let Some(s) = w.play(answer, guesser) {
            println!("Guessed {} in {}", answer, s);
        } else {
            eprintln!("failed to guess!");
        }
    }
}
