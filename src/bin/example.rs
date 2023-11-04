use scoreboard::*;

fn main() {
    println!("Hello World!");

    let mut score_board = ScoreBoard::new();
    let _ = score_board.start_match(String::from("China"), String::from("Uzbekistan"));
}