
const GAMES: &str = include_str!("./../resources/answers.txt");

fn main() {
    
    let w = oddvar::Wordle::new();

    for answer in GAMES.split_whitespace() {
        let answer_b: oddvar::Word = answer
            .as_bytes()
            .try_into()
            .expect("All answers are 5 letters");
        let guesser = oddvar::algorithms::Naive::new();
        
        if let Some(s) = w.play(answer_b, guesser) {
            println!("Guessed {} in {}", answer, s);
        } else {
            eprintln!("failed to guess!");
        }
    }
}
