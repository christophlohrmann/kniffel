use core::num;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

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
    fn is_already_written(&self, score_t: ScoreType) -> bool {
        match score_t {
            ScoreType::Ones => {
                match self.ones {
                    Some(_) => return true,
                    None => return false,
                };
            }
            ScoreType::Twos => {
                match self.twos {
                    Some(_) => return true,
                    None => return false,
                };
            }
            ScoreType::Threes => {
                match self.threes {
                    Some(_) => return true,
                    None => return false,
                };
            }
            ScoreType::Fours => {
                match self.fours {
                    Some(_) => return true,
                    None => return false,
                };
            }
            ScoreType::Fives => {
                match self.fives {
                    Some(_) => return true,
                    None => return false,
                };
            }
            ScoreType::Sixes => {
                match self.sixes {
                    Some(_) => return true,
                    None => return false,
                };
            }
            ScoreType::ThreeOfAKind => {
                match self.three_of_a_kind {
                    Some(_) => return true,
                    None => return false,
                };
            }
            ScoreType::FourOfAKind => {
                match self.four_of_a_kind {
                    Some(_) => return true,
                    None => return false,
                };
            }
            ScoreType::FullHouse => {
                match self.full_house {
                    Some(_) => return true,
                    None => return false,
                };
            }
            ScoreType::SmallStraight => {
                match self.small_straight {
                    Some(_) => return true,
                    None => return false,
                };
            }
            ScoreType::LargeStraight => {
                match self.large_straight {
                    Some(_) => return true,
                    None => return false,
                };
            }
            ScoreType::Yahtzee => {
                match self.yahtzee {
                    Some(_) => return true,
                    None => return false,
                };
            }
            ScoreType::Chance => {
                match self.chance {
                    Some(_) => return true,
                    None => return false,
                };
            }
        }
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

fn decide_keep_dice(numbers: &Vec<i32>, sheet: &ScoreSheet, round: i32) -> Vec<i32> {
    // return the indices of the numbers you want to keep
    let n_numbers = numbers.len();
    println!("enter indices of the dice you want to keep (0 to {}). Separate numbers by whitespace. Anything not an integer will be ignored.",
    n_numbers-1);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let indices: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    return indices;
}

fn decide_scoresheet_update(numbers: &Vec<i32>, sheet: &ScoreSheet) -> Result<ScoreType, String> {
    println!("your numbers are {numbers:?}");
    println!("your sheet so far is {sheet:?}");
    println!("pick score type to write to. For upper half type 1-6. For lower half type one of fh, tk, fk, ss, ls, y, c.
    If your numbers do not fulfil the shape criterion, the score type will be scratched");
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

fn validate_kept_indices(kept_indices: &Vec<i32>) -> Result<bool, String> {
    if kept_indices.len() > 5 {
        return Err(String::from("picked too many indices"));
    };
    for idx in kept_indices {
        if *idx as usize >= 5 {
            return Err(String::from("index out of bounds"));
        };
    }
    let unique_elements: HashSet<i32> = kept_indices.iter().cloned().collect();
    if unique_elements.len() != kept_indices.len() {
        return Err(String::from("indices not unique"));
    };

    return Ok(true);
}

fn vecs_elementwise_equal(vec1: &Vec<i32>, vec2: &Vec<i32>) -> bool {
    if vec1.len() != vec2.len() {
        return false;
    }
    return vec1.iter().zip(vec2).filter(|&(a, b)| a == b).count() == vec1.len();
}

fn count_unique_elements(vec: &Vec<i32>) -> HashMap<i32, usize> {
    let mut counts = HashMap::new();
    for &value in vec.iter() {
        counts
            .entry(value)
            .and_modify(|counter| *counter += 1)
            .or_insert(0);
    }
    return counts;
}

fn detect_large_straight(vec: &Vec<i32>) -> bool {
    let mut vec_mut: Vec<&i32> = vec.into_iter().collect();
    vec_mut.sort();
    return vecs_elementwise_equal(vec, &Vec::from([1, 2, 3, 4, 5]))
        || vecs_elementwise_equal(vec, &Vec::from([2, 3, 4, 5, 6]));
}

fn detect_small_straight(vec: &Vec<i32>) -> bool {
    //allow large straight too
    let unique_counts = count_unique_elements(vec);
    let mut unique_numbers: Vec<i32> = unique_counts
        .into_iter()
        .map(|(key, _val)| key as i32)
        .collect();
    unique_numbers.sort();

    return vecs_elementwise_equal(&unique_numbers, &Vec::from([1, 2, 3, 4, 5]))
        || vecs_elementwise_equal(&unique_numbers, &Vec::from([1, 2, 3, 4, 6]))
        || vecs_elementwise_equal(&unique_numbers, &Vec::from([2, 3, 4, 5, 6]))
        || vecs_elementwise_equal(&unique_numbers, &Vec::from([1, 3, 4, 5, 6]))
        || vecs_elementwise_equal(&unique_numbers, &Vec::from([1, 2, 3, 4]))
        || vecs_elementwise_equal(&unique_numbers, &Vec::from([2, 3, 4, 5]))
        || vecs_elementwise_equal(&unique_numbers, &Vec::from([3, 4, 5, 6]));
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
            let unique_counts = count_unique_elements(numbers);
            let mut unique_vals: Vec<i32> = unique_counts
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
            let unique_counts = count_unique_elements(numbers);
            let mut unique_vals: Vec<i32> = unique_counts
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
            let unique_counts = count_unique_elements(numbers);
            let mut unique_vals: Vec<i32> = unique_counts
                .into_iter()
                .map(|(_key, val)| val as i32)
                .collect();
            unique_vals.sort();
            let res = if vecs_elementwise_equal(&unique_vals, &Vec::from([2, 3])) {
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
            let unique_counts = count_unique_elements(numbers);
            let res = if unique_counts.len() == 1 { 50 } else { 0 };
            sheet.yahtzee = Some(res);
        }
        ScoreType::Chance => sheet.chance = Some(numbers.iter().sum()),
    }
}

fn set_slice_from_vec(dest: &mut Vec<i32>, src: Vec<i32>, start_index: usize) {
    // Check if the starting index is within bounds
    if start_index + src.len() > dest.len() {
        panic!("Source vector is too large for destination vector at the given index.");
    }

    // Copy the elements from src to the appropriate slice in dest
    dest[start_index..start_index + src.len()].copy_from_slice(&src);
}

fn play_turn(dice: &mut Dice, sheet: &mut ScoreSheet) {
    let mut kept_numbers: Vec<i32> = Vec::with_capacity(5);

    // round 1 and 2: throw and pick
    for round in 0..2 {
        println!("throw {:?}", round);
        // numbers to choose from = numbers kept from last round + random new ones
        let mut numbers: Vec<i32> = vec![0; 5];
        let mut n_kept_numbers = kept_numbers.len();
        set_slice_from_vec(&mut numbers, kept_numbers, 0);
        set_slice_from_vec(
            &mut numbers,
            dice.gen_numbers(5 - n_kept_numbers),
            n_kept_numbers,
        );
        println!("dice {numbers:?}");
        let mut kept_indices: Vec<i32> = Vec::new();
        loop {
            kept_indices = decide_keep_dice(&numbers, sheet, round);
            let validation_result = validate_kept_indices(&kept_indices);
            match validation_result {
                Ok(_) => {
                    println!("valid choice");
                    break;
                }
                Err(e) => println!("invalid choice. reason: {e}"),
            }
        }
        n_kept_numbers = kept_indices.len();
        println!("keeping dice no. {:?}", kept_indices);
        kept_numbers = Vec::with_capacity(n_kept_numbers);
        for idx in kept_indices {
            kept_numbers.push(numbers[idx as usize]);
        }

        if n_kept_numbers == 5 {
            break;
        }
    }

    //round 3: pick all remaining numbers randomly
    let n_kept_numbers = kept_numbers.len();
    let mut numbers: Vec<i32> = vec![0; 5];
    set_slice_from_vec(&mut numbers, kept_numbers, 0);
    set_slice_from_vec(
        &mut numbers,
        dice.gen_numbers(5 - n_kept_numbers),
        n_kept_numbers,
    );

    let mut chosen_score_t: ScoreType = ScoreType::Ones;
    loop {
        let choice_res = decide_scoresheet_update(&numbers, sheet);
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

    println!("sheet is {:?}", sheet);
}

fn main() {
    let mut dice = Dice::new();
    let mut sheet = ScoreSheet {
        ..Default::default()
    };

    for turn in 0..13 {
        println!("turn {turn}");
        play_turn(&mut dice, &mut sheet);
        println!("")
    }
    println!("final result: {:?}", sheet.sum());
}
