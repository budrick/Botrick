fn main() {
    let mut g = werdle::Game::new();
    let guessres = g.guess("qwert").unwrap();
    g.print_werd();
    println!("{:#?}", guessres);
    println!("{:?}", g.guessed_letters());
    println!("{:?}", g.is_finished());
}