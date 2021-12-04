use crate::file;

pub fn run(part: u8) {
    let exo = 3;
    let input = file::read(exo);
    let result = match part {
        1 => run1(&input),
        _ => run2(&input),
    };
    file::write(exo, part, &result.to_string());
}

fn run1(input: &str) -> usize {
    let report = parse_report(input);
    let mut gamma_str = String::from("");
    let mut epsilon_str = String::from("");

    for i in 0..report[0].len() {
        let mut one_counter = 0;
        for j in 0..report.len() {
            if report[j][i] == '1' {
                one_counter += 1;
            };
        }
        if one_counter * 2 > report.len() {
            gamma_str.push_str("1");
            epsilon_str.push_str("0");
        } else {
            gamma_str.push_str("0");
            epsilon_str.push_str("1");
        }
    }

    let gamma = usize::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon_str, 2).unwrap();

    gamma * epsilon
}

fn run2(input: &str) -> usize {
    let mut report = parse_report(input);
    let mut column = 0;

    while report.len() > 1 {
        let mut one_counter = 0;
        for i in 0..report.len() {
            if report[i][column] == '1' {
                one_counter += 1;
            };
        }
        let to_keep = if one_counter * 2 >= report.len() {
            '1'
        } else {
            '0'
        };
        report = report
            .into_iter()
            .filter(|line| line[column] == to_keep)
            .collect::<Vec<Vec<char>>>();
        column += 1;
    }

    let oxygen_str = report.pop().unwrap().into_iter().collect::<String>();
    let oxygen = usize::from_str_radix(&oxygen_str, 2).unwrap();

    let mut report = parse_report(input);
    let mut column = 0;

    while report.len() > 1 {
        let mut zero_counter = 0;
        for i in 0..report.len() {
            if report[i][column] == '0' {
                zero_counter += 1;
            };
        }
        let to_keep = if zero_counter * 2 <= report.len() {
            '0'
        } else {
            '1'
        };
        report = report
            .into_iter()
            .filter(|line| line[column] == to_keep)
            .collect::<Vec<Vec<char>>>();
        column += 1;
    }

    let co2_str = report.pop().unwrap().into_iter().collect::<String>();
    let co2 = usize::from_str_radix(&co2_str, 2).unwrap();

    oxygen * co2
}

fn parse_report(input: &str) -> Vec<Vec<char>> {
    input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run11() {
        assert_eq!(run1("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"), 198);
    }

    #[test]
    fn run21() {
        assert_eq!(run2("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"), 230);
    }
}
