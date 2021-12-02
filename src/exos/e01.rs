use crate::file;

pub fn run(part: u8) {
    let exo = 1;
    let input = file::read(exo);
    let result = match part {
        1 => run1(&input),
        _ => run2(&input),
    };
    file::write(exo, part, &result.to_string());
}

fn run1(input: &str) -> usize {
    let ms = measurements(input);

    let mut increase = 0;
    let mut current = ms[0];

    for i in 1..ms.len() {
        let m = ms[i];
        if m > current {
            increase += 1;
        }
        current = m;
    }

    increase
}

fn run2(input: &str) -> usize {
    let ms = measurements(input);

    let mut increase = 0;
    let mut current = ms[0] + ms[1] + ms[2];

    for i in 1..ms.len() - 2 {
        let sum = ms[i] + ms[i + 1] + ms[i + 2];
        if sum > current {
            increase += 1;
        }
        current = sum;
    }

    increase
}

fn measurements(input: &str) -> Vec<usize> {
    input
        .split("\n")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run11() {
        assert_eq!(run1("199\n200\n208\n210\n200\n207\n240\n269\n260\n263"), 7);
    }

    #[test]
    fn run21() {
        assert_eq!(run2("607\n618\n618\n617\n647\n716\n769\n792"), 5);
    }
}
