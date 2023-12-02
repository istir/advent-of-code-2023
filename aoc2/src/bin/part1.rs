#[derive(Debug)]
struct Game {
    id: i32,
    scores: Vec<Scores>,
}
#[derive(Debug)]
struct Scores {
    red: i32,
    green: i32,
    blue: i32,
}

impl Game {
    fn calculate_score(&self) -> Scores {
        let scores = Scores {
            red: 0,
            green: 0,
            blue: 0,
        };
        let calculated_scores = self.scores.iter().fold(scores, |acc, val| Scores {
            red: acc.red + val.red,
            green: acc.green + val.green,
            blue: acc.blue + val.blue,
        });
        return calculated_scores;
    }

    fn is_game_possible(&self) -> bool {
        const MAX_POSSIBLE_RED: i32 = 12;
        const MAX_POSSIBLE_GREEN: i32 = 13;
        const MAX_POSSIBLE_BLUE: i32 = 14;
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for score in &self.scores {
            max_red = max_red.max(score.red);
            max_green = max_green.max(score.green);
            max_blue = max_blue.max(score.blue);
        }

        return max_red <= MAX_POSSIBLE_RED
            && max_green <= MAX_POSSIBLE_GREEN
            && max_blue <= MAX_POSSIBLE_BLUE;
    }
}

fn main() {
    let input = include_str!("./input1.txt");
    let res = part1(input);
    println!("{}", res);
}

fn part1(input: &str) -> i32 {
    let parts: Vec<&str> = input.split("\n").filter(|item| !item.is_empty()).collect();

    let data: Vec<Game> = parts
        .iter()
        .map(|part| {
            return parse_game(part);
        })
        .collect();
    data.iter().for_each(|game| {
        println!("Score: {:?}", game.calculate_score());
        println!("Is possible: {:?}", game.is_game_possible());
    });
    let sum = calculate_possible_game_sum(data);
    println!("Sum: {}", sum);
    return sum;
}

fn parse_game(game: &str) -> Game {
    let binding = game.to_ascii_lowercase().replace("game ", "");
    let mut parts = binding.split(": ");
    let id = parts.next().unwrap_or("0").parse::<i32>().unwrap_or(0);
    let game_data = parts.next().unwrap_or("");

    let game_scores = game_data
        .split("; ")
        .map(|score| parse_game_score(score))
        .collect();

    let game = Game {
        id: id,
        scores: game_scores,
    };
    return game;
}

fn parse_game_score(game_scores: &str) -> Scores {
    let mut score_points = Scores {
        red: 0,
        green: 0,
        blue: 0,
    };
    game_scores.split(", ").for_each(|game_part| {
        let mut parts = game_part.split_ascii_whitespace();
        let score = parts.next().unwrap_or("0").parse::<i32>().unwrap_or(0);
        let color = parts.next().unwrap_or("");
        match color {
            "red" => score_points.red += score,
            "green" => score_points.green += score,
            "blue" => score_points.blue += score,
            _ => (),
        }
    });
    return score_points;
}

fn calculate_possible_game_sum(games: Vec<Game>) -> i32 {
    let count: i32 = games
        .iter()
        .filter(|game| game.is_game_possible())
        .map(|game| game.id)
        .sum();
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_test() {
        let result = part1(include_str!("./test_input1.txt"));

        assert_eq!(result, 8);
    }
}
