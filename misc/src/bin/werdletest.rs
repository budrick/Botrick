fn main() {
    let mut g = werdle::Game::new();
    let guessres = g.guess("qwert").unwrap();
    g.print_werd();
    println!("{:#?}", guessres);
    println!("Guessed letters this game: {:?}", g.guessed_letters());
    println!("Finished? {:?}", g.is_finished());
}
