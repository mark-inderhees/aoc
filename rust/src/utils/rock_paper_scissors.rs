/// A game of rock paper scissors, has 3 play types.
pub enum ItemType {
    Rock,
    Paper,
    Scissors,
}

/// A game has 3 result types.
pub enum ResultType {
    Loss,
    Tie,
    Win,
}

/// Given what two players have in rock paper scissors, give the result
pub fn determine_result(me: &ItemType, them: &ItemType) -> ResultType {
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

/// Given what one player has and a result in rock paper scissors, give what the other player must have
pub fn determine_type_from_result(player1: &ItemType, result: &ResultType) -> ItemType {
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
