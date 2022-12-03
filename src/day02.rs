const SCORE_LOST: i32 = 0;
const SCORE_DRAW: i32 = 3;
const SCORE_WON: i32 = 6;

pub fn first(input: String) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let winners = vec!["A Y", "B Z", "C X"];
    let draws = vec!["A X", "B Y", "C Z"];
    let my_shapes = "XYZ";
    let my_scores_by_shape = vec![1, 2, 3];
    let mut total_score = 0;

    for round_str in lines {
        if round_str.len() != 3 {
            continue;
        }

        let is_winner = winners.contains(&round_str);
        let is_draw = draws.contains(&round_str);
        let my_play = round_str.chars().nth(2).unwrap();

        let round_score = if is_winner {
            SCORE_WON
        } else if is_draw {
            SCORE_DRAW
        } else {
            SCORE_LOST
        };

        let shape_score = my_scores_by_shape[my_shapes.chars().position(|c| c == my_play).unwrap()];

        total_score += round_score + shape_score;
    }

    return format!("{}", total_score);
}

pub fn day02(input: String) -> String {
    let first_strategy: String = first(String::from(input.as_str()));

    let lines: Vec<&str> = input.lines().collect();
    let losses = vec!["A X", "B X", "C X"];
    let draws = vec!["A Y", "B Y", "C Y"];
    let winners = vec!["A Z", "B Z", "C Z"];
    let lose_scores = vec![3, 1, 2];
    let draw_scores = vec![1, 2, 3];
    let win_scores = vec![2, 3, 1];
    let mut total_score = 0;

    for round_str in lines {
        if round_str.len() != 3 {
            continue;
        }

        let is_winner = winners.contains(&round_str);
        let is_draw = draws.contains(&round_str);

        let round_score = if is_winner {
            SCORE_WON + win_scores[winners.iter().position(|s| s == &round_str).unwrap()]
        } else if is_draw {
            SCORE_DRAW + draw_scores[draws.iter().position(|s| s == &round_str).unwrap()]
        } else {
            SCORE_LOST + lose_scores[losses.iter().position(|s| s == &round_str).unwrap()]
        };

        total_score += round_score;
    }

    return format!(
        "First strategy: {}\nReal strategy: {}",
        first_strategy, total_score
    );
}
