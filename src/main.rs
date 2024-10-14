use kniffel::engine;
use std::collections::HashMap;

fn main() {
    let mut dice = engine::Dice::new();

    let players = vec!["chris", "katja"];

    let mut score_sheets: HashMap<String, engine::ScoreSheet> = HashMap::new();
    for player in players {
        score_sheets.insert(String::from(player), engine::ScoreSheet::new());
    }

    for turn in 1..14 {
        println!("Start of turn {turn}. Score so far");
        engine::print_all_score_sheets(&score_sheets);
        println!();

        for (player, sheet_ref) in &mut score_sheets {
            println!("turn {turn} for player {player}");
            engine::play_turn(&mut dice, sheet_ref);
            println!("");
        }
    }
    println!("final result:");
    engine::print_all_score_sheets(&mut score_sheets);
}
