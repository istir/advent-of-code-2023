fn main() {
    let input = include_str!("./input1.txt");
    let res = part1(input);
    println!("{}", res);
}

fn part1(input: &str) -> i32 {
    let parts: Vec<&str> = input.split("\n").filter(|item| !item.is_empty()).collect();

    let data: Vec<(i32, i32)> = parts
        .iter()
        .map(|part| {
            return extract_first_last_number(part);
        })
        .collect();
    return add_values_to_sum(data);
}

fn extract_first_last_number(data: &str) -> (i32, i32) {
    let characters = data.chars();

    let numbers: Vec<i32> = characters
        .filter_map(|character| {
            if character.is_digit(10) {
                Some(character.to_digit(10).unwrap() as i32)
            } else {
                None
            }
        })
        .collect();
    let first = numbers[0];

    let last = numbers[numbers.len() - 1];

    return (first, last);
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
        let result = part1(include_str!("./test_input1.txt"));

        assert_eq!(result, 142);
    }
}
