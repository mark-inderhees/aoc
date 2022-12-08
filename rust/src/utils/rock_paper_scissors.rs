pub enum ItemType {
    Rock,
    Paper,
    Scissors,
}

pub enum ResultType {
    Loss,
    Tie,
    Win,
}

pub fn get_result(me: &ItemType, them: &ItemType) -> ResultType {
    match (me, them) {
        (ItemType::Rock, ItemType::Rock) => ResultType::Tie,
        (ItemType::Rock, ItemType::Paper) => ResultType::Loss,
        (ItemType::Rock, ItemType::Scissors) => ResultType::Win,
        (ItemType::Paper, ItemType::Rock) => ResultType::Win,
        (ItemType::Paper, ItemType::Paper) => ResultType::Tie,
        (ItemType::Paper, ItemType::Scissors) => ResultType::Loss,
        (ItemType::Scissors, ItemType::Rock) => ResultType::Loss,
        (ItemType::Scissors, ItemType::Paper) => ResultType::Win,
        (ItemType::Scissors, ItemType::Scissors) => ResultType::Tie,
    }
}

pub fn get_type_from_result(player1: &ItemType, result: &ResultType) -> ItemType {
    match (player1, result) {
        (ItemType::Rock, ResultType::Loss) => ItemType::Scissors,
        (ItemType::Rock, ResultType::Tie) => ItemType::Rock,
        (ItemType::Rock, ResultType::Win) => ItemType::Paper,
        (ItemType::Paper, ResultType::Loss) => ItemType::Rock,
        (ItemType::Paper, ResultType::Tie) => ItemType::Paper,
        (ItemType::Paper, ResultType::Win) => ItemType::Scissors,
        (ItemType::Scissors, ResultType::Loss) => ItemType::Paper,
        (ItemType::Scissors, ResultType::Tie) => ItemType::Scissors,
        (ItemType::Scissors, ResultType::Win) => ItemType::Rock,
    }
}