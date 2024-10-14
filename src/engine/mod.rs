use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::HashMap;
use std::io;

mod utils;

#[derive(Debug, Clone, Copy)]
enum ScoreType {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    ThreeOfAKind,
    FourOfAKind,
    FullHouse,
    SmallStraight,
    LargeStraight,
    Yahtzee,
    Chance,
}

#[derive(Default, Debug)]
struct ScoreSheet {
    ones: Option<i32>,
    twos: Option<i32>,
    threes: Option<i32>,
    fours: Option<i32>,
    fives: Option<i32>,
    sixes: Option<i32>,
    three_of_a_kind: Option<i32>,
    four_of_a_kind: Option<i32>,
    full_house: Option<i32>,
    small_straight: Option<i32>,
    large_straight: Option<i32>,
    yahtzee: Option<i32>,
    chance: Option<i32>,
}

impl ScoreSheet {
    fn new() -> ScoreSheet {
        ScoreSheet {
            ..Default::default()
        }
    }
    fn sum(&self) -> i32 {
        let upper_sum = self.ones.unwrap_or_default()
            + self.twos.unwrap_or_default()
            + self.threes.unwrap_or_default()
            + self.fours.unwrap_or_default()
            + self.fives.unwrap_or_default()
            + self.sixes.unwrap_or_default();
        let bonus = if upper_sum >= 63 { 35 } else { 0 };
        let lower_sum = self.three_of_a_kind.unwrap_or_default()
            + self.four_of_a_kind.unwrap_or_default()
            + self.full_house.unwrap_or_default()
            + self.small_straight.unwrap_or_default()
            + self.large_straight.unwrap_or_default()
            + self.yahtzee.unwrap_or_default()
            + self.chance.unwrap_or_default();
        return upper_sum + bonus + lower_sum;
    }

    fn get_score(&self, score_t: ScoreType) -> Option<i32> {
        match score_t {
            ScoreType::Ones => return self.ones,
            ScoreType::Twos => return self.twos,
            ScoreType::Threes => return self.threes,
            ScoreType::Fours => return self.fours,
            ScoreType::Fives => return self.fives,
            ScoreType::Sixes => return self.sixes,
            ScoreType::ThreeOfAKind => return self.three_of_a_kind,
            ScoreType::FourOfAKind => return self.four_of_a_kind,
            ScoreType::FullHouse => return self.full_house,
            ScoreType::SmallStraight => return self.small_straight,
            ScoreType::LargeStraight => return self.large_straight,
            ScoreType::Yahtzee => return self.yahtzee,
            ScoreType::Chance => return self.chance,
        }
    }

    fn is_already_written(&self, score_t: ScoreType) -> bool {
        match self.get_score(score_t) {
            Some(_) => return true,
            None => return false,
        }
    }
}

fn print_all_score_sheets(player_sheets: &mut HashMap<String, ScoreSheet>) {
    // TODO: Sort out how this function can be called with immutable refs in the main game loop.
    let score_type_names = vec![
        "Ones",
        "Twos",
        "Threes",
        "Fours",
        "Fives",
        "Sixes",
        "Three of a Kind",
        "Four of a Kind",
        "Full House",
        "Small Straight",
        "Large Straight",
        "Yahtzee",
        "Chance",
    ];
    let score_types = vec![
        ScoreType::Ones,
        ScoreType::Twos,
        ScoreType::Threes,
        ScoreType::Fours,
        ScoreType::Fives,
        ScoreType::Sixes,
        ScoreType::ThreeOfAKind,
        ScoreType::FourOfAKind,
        ScoreType::FullHouse,
        ScoreType::SmallStraight,
        ScoreType::LargeStraight,
        ScoreType::Yahtzee,
        ScoreType::Chance,
    ];

    // Print the header row: Player names as columns
    print!("{:<25}", "");
    for player_name in player_sheets.keys() {
        print!("{:<15}", player_name);
    }
    println!();

    // Print each category with the corresponding score for each player
    for (i, category) in score_type_names.iter().enumerate() {
        print!("{:<25}", category);
        for sheet in player_sheets.values() {
            print!(
                "{:<15}",
                sheet
                    .get_score(score_types[i])
                    .map_or(" ".to_string(), |v| v.to_string())
            );
        }

        println!();
    }
}

struct Dice {
    rng: ThreadRng,
}

impl Dice {
    fn new() -> Dice {
        Dice {
            rng: rand::thread_rng(),
        }
    }

    fn gen_numbers(&mut self, n: usize) -> Vec<i32> {
        let mut res = Vec::with_capacity(n);
        for _i in 0..n {
            res.push(self.rng.gen_range(1..=6));
        }
        res.sort();
        return res;
    }
}

fn decide_keep_dice(numbers: &Vec<i32>) -> Vec<i32> {
    // return the numbers (not their indices) you want to keep
    println!("Your numbers are {numbers:?}");
    println!("Enter the numbers you want to keep and press enter. Separate numbers by whitespace. Anything not an integer will be ignored. Press enter without any numbers if you want to reroll all dice");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let keep_numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    return keep_numbers;
}

fn decide_scoresheet_update(numbers: &Vec<i32>) -> Result<ScoreType, String> {
    println!("your numbers are {numbers:?}");
    println!("pick score type to write to. For upper half type 1-6. For lower half type one of fh, tk, fk, ss, ls, y, c. If your numbers do not fulfil the shape criterion, the score type will be scratched");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.to_lowercase().trim() {
        "1" => return Ok(ScoreType::Ones),
        "2" => return Ok(ScoreType::Twos),
        "3" => return Ok(ScoreType::Threes),
        "4" => return Ok(ScoreType::Fours),
        "5" => return Ok(ScoreType::Fives),
        "6" => return Ok(ScoreType::Sixes),
        "tk" => return Ok(ScoreType::ThreeOfAKind),
        "fk" => return Ok(ScoreType::FourOfAKind),
        "fh" => return Ok(ScoreType::FullHouse),
        "ss" => return Ok(ScoreType::SmallStraight),
        "ls" => return Ok(ScoreType::LargeStraight),
        "y" => return Ok(ScoreType::Yahtzee),
        "c" => return Ok(ScoreType::Chance),
        _ => return Err(input),
    }
}

fn validate_kept_numbers(kept_numbers: &Vec<i32>, numbers: &Vec<i32>) -> Result<(), String> {
    if kept_numbers.len() > numbers.len() {
        return Err(String::from("picked too many numbers"));
    };
    for num in kept_numbers {
        if *num > 6 || *num < 1 {
            return Err(String::from(format!("Invalid number {num}")));
        };
    }
    let unique_kept = utils::count_unique_elements(&kept_numbers);
    let unique_nums = utils::count_unique_elements(&numbers);

    for (kept_num, count) in &unique_kept {
        let available = unique_nums.get(kept_num).unwrap_or(&0);
        if count > available {
            return Err(String::from(format!(
                "Picked {count} {kept_num}'s but you only have {available}"
            )));
        }
    }

    return Ok(());
}

fn detect_large_straight(vec: &Vec<i32>) -> bool {
    let mut vec_mut: Vec<i32> = vec.clone();
    vec_mut.sort();
    return utils::vecs_elementwise_equal(&vec_mut, &Vec::from([1, 2, 3, 4, 5]))
        || utils::vecs_elementwise_equal(&vec_mut, &Vec::from([2, 3, 4, 5, 6]));
}

fn detect_small_straight(vec: &Vec<i32>) -> bool {
    //allow large straight too
    let unique_counts = utils::count_unique_elements(vec);
    let mut unique_numbers: Vec<i32> = unique_counts
        .into_iter()
        .map(|(key, _val)| key as i32)
        .collect();
    unique_numbers.sort();

    return utils::vecs_elementwise_equal(&unique_numbers, &Vec::from([1, 2, 3, 4, 5]))
        || utils::vecs_elementwise_equal(&unique_numbers, &Vec::from([1, 2, 3, 4, 6]))
        || utils::vecs_elementwise_equal(&unique_numbers, &Vec::from([2, 3, 4, 5, 6]))
        || utils::vecs_elementwise_equal(&unique_numbers, &Vec::from([1, 3, 4, 5, 6]))
        || utils::vecs_elementwise_equal(&unique_numbers, &Vec::from([1, 2, 3, 4]))
        || utils::vecs_elementwise_equal(&unique_numbers, &Vec::from([2, 3, 4, 5]))
        || utils::vecs_elementwise_equal(&unique_numbers, &Vec::from([3, 4, 5, 6]));
}

fn update_score_sheet(sheet: &mut ScoreSheet, score_t: ScoreType, numbers: &Vec<i32>) {
    match score_t {
        ScoreType::Ones => {
            sheet.ones = Some(1 * numbers.iter().filter(|&&x| x == 1).count() as i32);
        }
        ScoreType::Twos => {
            sheet.twos = Some(2 * numbers.iter().filter(|&&x| x == 2).count() as i32);
        }
        ScoreType::Threes => {
            sheet.threes = Some(3 * numbers.iter().filter(|&&x| x == 3).count() as i32);
        }
        ScoreType::Fours => {
            sheet.fours = Some(4 * numbers.iter().filter(|&&x| x == 4).count() as i32);
        }
        ScoreType::Fives => {
            sheet.fives = Some(5 * numbers.iter().filter(|&&x| x == 5).count() as i32);
        }
        ScoreType::Sixes => {
            sheet.sixes = Some(6 * numbers.iter().filter(|&&x| x == 6).count() as i32);
        }
        ScoreType::ThreeOfAKind => {
            let unique_counts = utils::count_unique_elements(numbers);
            let unique_vals: Vec<i32> = unique_counts
                .into_iter()
                .map(|(_key, val)| val as i32)
                .collect();
            let res = if *(unique_vals.iter().max().unwrap_or(&0)) >= 3 {
                numbers.iter().sum()
            } else {
                0
            };
            sheet.three_of_a_kind = Some(res);
        }
        ScoreType::FourOfAKind => {
            let unique_counts = utils::count_unique_elements(numbers);
            let unique_vals: Vec<i32> = unique_counts
                .into_iter()
                .map(|(_key, val)| val as i32)
                .collect();
            let res = if *(unique_vals.iter().max().unwrap_or(&0)) >= 4 {
                numbers.iter().sum()
            } else {
                0
            };
            sheet.four_of_a_kind = Some(res);
        }
        ScoreType::FullHouse => {
            let unique_counts = utils::count_unique_elements(numbers);
            let mut unique_vals: Vec<i32> = unique_counts
                .into_iter()
                .map(|(_key, val)| val as i32)
                .collect();
            unique_vals.sort();
            let res = if utils::vecs_elementwise_equal(&unique_vals, &Vec::from([2, 3])) {
                25
            } else {
                0
            };
            sheet.full_house = Some(res);
        }
        ScoreType::SmallStraight => {
            let res = if detect_small_straight(numbers) {
                30
            } else {
                0
            };
            sheet.small_straight = Some(res);
        }
        ScoreType::LargeStraight => {
            let res = if detect_large_straight(numbers) {
                40
            } else {
                0
            };
            sheet.large_straight = Some(res);
        }
        ScoreType::Yahtzee => {
            let unique_counts = utils::count_unique_elements(numbers);
            let res = if unique_counts.len() == 1 { 50 } else { 0 };
            sheet.yahtzee = Some(res);
        }
        ScoreType::Chance => sheet.chance = Some(numbers.iter().sum()),
    }
}

fn play_turn(dice: &mut Dice, sheet: &mut ScoreSheet) {
    // Handle one turn (up to three dice throws) for one player.
    let mut kept_numbers: Vec<i32> = Vec::with_capacity(5);

    // round 1 and 2: throw and pick
    for round in 0..2 {
        println!("throw {:?}", round);
        // numbers to choose from = numbers kept from last round + random new ones
        let mut numbers: Vec<i32> = vec![0; 5];
        let n_kept_numbers = kept_numbers.len();
        utils::set_slice_from_vec(&mut numbers, kept_numbers, 0);
        utils::set_slice_from_vec(
            &mut numbers,
            dice.gen_numbers(5 - n_kept_numbers),
            n_kept_numbers,
        );
        loop {
            kept_numbers = decide_keep_dice(&numbers);
            let validation_result = validate_kept_numbers(&kept_numbers, &numbers);
            match validation_result {
                Ok(_) => {
                    println!("valid choice, keeping {kept_numbers:?}");
                    break;
                }
                Err(e) => println!("invalid choice. reason: {e}"),
            }
        }
        if kept_numbers.len() == 5 {
            break;
        }
    }

    //round 3: pick all remaining numbers randomly
    let n_kept_numbers = kept_numbers.len();
    let mut numbers: Vec<i32> = vec![0; 5];
    utils::set_slice_from_vec(&mut numbers, kept_numbers, 0);
    utils::set_slice_from_vec(
        &mut numbers,
        dice.gen_numbers(5 - n_kept_numbers),
        n_kept_numbers,
    );

    let mut chosen_score_t: ScoreType;
    loop {
        let choice_res = decide_scoresheet_update(&numbers);
        match choice_res {
            Ok(x) => chosen_score_t = x,
            Err(e) => {
                println!("invalid input {e}. try again.");
                continue;
            }
        }

        if !sheet.is_already_written(chosen_score_t) {
            println!("writing {chosen_score_t:?}");
            break;
        } else {
            println!("invalid choice: {chosen_score_t:?} already written")
        }
    }

    update_score_sheet(sheet, chosen_score_t, &numbers);
}

fn get_player_names() -> Vec<String> {
    // get player names from user input. Chooses default name if input is empty.
    println!("Enter player names separated by whitespace");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut player_names: Vec<String> = input.trim().split_whitespace().map(String::from).collect();
    if player_names.is_empty() {
        player_names = vec![String::from("default_name")];
    }
    return player_names;
}

pub fn play_game() {
    {
        let mut dice = Dice::new();

        let players = get_player_names();

        let mut score_sheets: HashMap<String, ScoreSheet> = HashMap::new();
        for player in players {
            score_sheets.insert(String::from(player), ScoreSheet::new());
        }

        for turn in 1..14 {
            println!("Start of turn {turn}. Scores so far are");
            print_all_score_sheets(&mut score_sheets);
            println!();
            for (player, sheet_ref) in &mut score_sheets {
                println!("turn {turn} for player {player}.");
                play_turn(&mut dice, sheet_ref);
                println!();
            }
        }
        println!("final result:");
        for (player, sheet) in score_sheets {
            let sum = sheet.sum();
            println!("{player}: {sum}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod update_scores {
        use super::{update_score_sheet, ScoreSheet};

        #[test]
        fn ones() {
            let mut sheet = ScoreSheet::new();
            let dice = vec![1, 1, 3, 1, 2];
            update_score_sheet(&mut sheet, super::ScoreType::Ones, &dice);
            assert_eq!(sheet.ones.unwrap(), 3);
        }
        #[test]
        fn twos() {
            let mut sheet = ScoreSheet::new();
            let dice = vec![1, 2, 3, 1, 2];
            update_score_sheet(&mut sheet, super::ScoreType::Twos, &dice);
            assert_eq!(sheet.twos.unwrap(), 4);
        }
        #[test]
        fn threes() {
            let mut sheet = ScoreSheet::new();
            let dice = vec![3, 1, 3, 1, 2];
            update_score_sheet(&mut sheet, super::ScoreType::Threes, &dice);
            assert_eq!(sheet.threes.unwrap(), 6);
        }
        #[test]
        fn fours() {
            let mut sheet = ScoreSheet::new();
            let dice = vec![1, 4, 4, 1, 4];
            update_score_sheet(&mut sheet, super::ScoreType::Fours, &dice);
            assert_eq!(sheet.fours.unwrap(), 12);
        }
        #[test]
        fn fives() {
            let mut sheet = ScoreSheet::new();
            let dice = vec![5, 5, 3, 1, 5];
            update_score_sheet(&mut sheet, super::ScoreType::Fives, &dice);
            assert_eq!(sheet.fives.unwrap(), 15);
        }
        #[test]
        fn sixes() {
            let mut sheet = ScoreSheet::new();
            let dice = vec![1, 1, 6, 1, 6];
            update_score_sheet(&mut sheet, super::ScoreType::Sixes, &dice);
            assert_eq!(sheet.sixes.unwrap(), 12);
        }
        #[test]
        fn three_of_a_kind() {
            let mut sheet = ScoreSheet::new();
            let dice_fulfill = vec![1, 1, 3, 1, 2];
            let dice_scratch = vec![1, 3, 3, 1, 2];
            update_score_sheet(&mut sheet, super::ScoreType::ThreeOfAKind, &dice_fulfill);
            assert_eq!(sheet.three_of_a_kind.unwrap(), 8);
            update_score_sheet(&mut sheet, super::ScoreType::ThreeOfAKind, &dice_scratch);
            assert_eq!(sheet.three_of_a_kind.unwrap(), 0);
        }
        #[test]
        fn four_of_a_kind() {
            let mut sheet = ScoreSheet::new();
            let dice_fulfill = vec![1, 1, 3, 1, 1];
            let dice_scratch = vec![1, 3, 3, 1, 1];
            update_score_sheet(&mut sheet, super::ScoreType::FourOfAKind, &dice_fulfill);
            assert_eq!(sheet.four_of_a_kind.unwrap(), 7);
            update_score_sheet(&mut sheet, super::ScoreType::FourOfAKind, &dice_scratch);
            assert_eq!(sheet.four_of_a_kind.unwrap(), 0);
        }
        #[test]
        fn full_house() {
            let mut sheet = ScoreSheet::new();
            let dice_fulfill = vec![1, 1, 3, 3, 1];
            let dice_scratch = vec![1, 3, 3, 1, 2];
            update_score_sheet(&mut sheet, super::ScoreType::FullHouse, &dice_fulfill);
            assert_eq!(sheet.full_house.unwrap(), 25);
            update_score_sheet(&mut sheet, super::ScoreType::FullHouse, &dice_scratch);
            assert_eq!(sheet.full_house.unwrap(), 0);
        }
        #[test]
        fn small_straight() {
            let mut legal_dice: Vec<Vec<i32>> = Vec::new();
            legal_dice.push(vec![1, 2, 3, 4, 5]);
            legal_dice.push(vec![1, 3, 2, 4, 5]);
            legal_dice.push(vec![1, 2, 3, 4, 1]);
            legal_dice.push(vec![3, 4, 5, 6, 1]);
            legal_dice.push(vec![3, 3, 4, 5, 6]);
            for throw in legal_dice {
                let mut sheet = ScoreSheet::new();
                update_score_sheet(&mut sheet, super::ScoreType::SmallStraight, &throw);
                assert_eq!(sheet.small_straight.unwrap(), 30);
            }

            let mut illegal_dice: Vec<Vec<i32>> = Vec::new();
            illegal_dice.push(vec![1, 1, 1, 2, 3]);
            illegal_dice.push(vec![1, 2, 3, 5, 6]);
            for throw in illegal_dice {
                let mut sheet = ScoreSheet::new();
                update_score_sheet(&mut sheet, super::ScoreType::SmallStraight, &throw);
                assert_eq!(sheet.small_straight.unwrap(), 0);
            }
        }
        #[test]
        fn large_straight() {
            let mut legal_dice: Vec<Vec<i32>> = Vec::new();
            legal_dice.push(vec![1, 2, 3, 4, 5]);
            legal_dice.push(vec![2, 3, 4, 5, 6]);
            legal_dice.push(vec![6, 5, 4, 2, 3]);
            for throw in legal_dice {
                let mut sheet = ScoreSheet::new();
                update_score_sheet(&mut sheet, super::ScoreType::LargeStraight, &throw);
                assert_eq!(sheet.large_straight.unwrap(), 40);
            }

            let mut illegal_dice: Vec<Vec<i32>> = Vec::new();
            illegal_dice.push(vec![1, 1, 1, 2, 3]);
            illegal_dice.push(vec![1, 2, 3, 5, 6]);
            illegal_dice.push(vec![1, 2, 3, 4, 6]);
            for throw in illegal_dice {
                let mut sheet = ScoreSheet::new();
                update_score_sheet(&mut sheet, super::ScoreType::LargeStraight, &throw);
                assert_eq!(sheet.large_straight.unwrap(), 0);
            }
        }
        #[test]
        fn yahtzee() {
            let legal_dice = vec![2, 2, 2, 2, 2];
            let illegal_dice = vec![2, 2, 2, 3, 2];
            let mut sheet = ScoreSheet::new();
            update_score_sheet(&mut sheet, super::ScoreType::Yahtzee, &legal_dice);
            assert_eq!(sheet.yahtzee.unwrap(), 50);
            update_score_sheet(&mut sheet, super::ScoreType::Yahtzee, &illegal_dice);
            assert_eq!(sheet.yahtzee.unwrap(), 0);
        }
        #[test]
        fn chance() {
            let legal_dice = vec![1, 2, 3, 4, 5];
            let mut sheet = ScoreSheet::new();
            update_score_sheet(&mut sheet, super::ScoreType::Chance, &legal_dice);
            assert_eq!(sheet.chance.unwrap(), 15);
        }
    }
}
