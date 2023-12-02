fn main() {
    let input = include_str!("./input1.txt");
    let res = part2(input);
    println!("{}", res);
    if res != 53340 {
        println!("Wrong");
    }
}

fn part2(input: &str) -> i32 {
    let parts: Vec<&str> = input.split("\n").filter(|item| !item.is_empty()).collect();

    let data: Vec<(i32, i32)> = parts
        .iter()
        .map(|part| {
            let response = extract_first_last_number(part);
            println!("input: {}, Numbers: {:?}", part, response);
            return response;
        })
        .collect();
    return add_values_to_sum(data);
}

fn extract_first_last_number(data: &str) -> (i32, i32) {
    let numbers = find_items(data.to_string());

    let first = numbers.first().unwrap_or(&0);
    let last = numbers.last().unwrap_or(&0);
    return (*first, *last);
}

fn find_items(data: String) -> Vec<i32> {
    let spelled_digits = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    let mut start: Vec<i32> = Vec::new();
    let mut end: Vec<i32> = Vec::new();
    let mut work_data = data.clone();
    while work_data.len() > 0 {
        spelled_digits.iter().for_each(|(search, result)| {
            if work_data.starts_with(search) {
                start.push(*result);
            }
            if work_data.ends_with(search) {
                end.push(*result);
            }
        });
        let mut chars = work_data.chars();
        chars.next();
        chars.next_back();
        work_data = chars.as_str().to_string();
    }
    end.reverse();
    start.append(&mut end);
    return start;
}

fn add_values_to_sum(values: Vec<(i32, i32)>) -> i32 {
    let v = values.iter();
    let sum: i32 = v.map(|value| value.0 * 10 + value.1).sum();
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_test() {
        let result = part2(include_str!("./test_input2.txt"));

        assert_eq!(result, 289);
    }
}
