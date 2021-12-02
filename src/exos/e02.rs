use crate::file;

pub fn run(part: u8) {
    let exo = 2;
    let input = file::read(exo);
    let result = match part {
        1 => run1(&input),
        _ => run2(&input),
    };
    file::write(exo, part, &result.to_string());
}

fn run1(input: &str) -> usize {
    let cs = commands(input);
    let mut sub = SubMarine::new();

    for c in cs {
        sub.run(c);
    }

    sub.depth * sub.position
}

fn run2(input: &str) -> usize {
    let cs = commands(input);
    let mut sub = SubMarine::new();

    for c in cs {
        sub.run_with_aim(c);
    }

    sub.depth * sub.position
}

fn commands(input: &str) -> Vec<Command> {
    input
        .split("\n")
        .map(|line| {
            let split = line.split(" ").collect::<Vec<&str>>();
            Command::from(split[0], split[1].parse::<usize>().unwrap())
        })
        .collect::<Vec<Command>>()
}

enum Command {
    Forward(usize),
    Up(usize),
    Down(usize),
}

impl Command {
    fn from(direction: &str, units: usize) -> Self {
        match direction {
            "forward" => Self::Forward(units),
            "up" => Self::Up(units),
            "down" => Self::Down(units),
            _ => unreachable!(),
        }
    }
}

struct SubMarine {
    aim: usize,
    depth: usize,
    position: usize,
}

impl SubMarine {
    fn new() -> Self {
        Self {
            aim: 0,
            depth: 0,
            position: 0,
        }
    }

    fn run(&mut self, command: Command) {
        match command {
            Command::Forward(units) => self.position += units,
            Command::Up(units) => self.depth -= units,
            Command::Down(units) => self.depth += units,
        }
    }

    fn run_with_aim(&mut self, command: Command) {
        match command {
            Command::Forward(units) => {
                self.position += units;
                self.depth += self.aim * units;
            }
            Command::Up(units) => self.aim -= units,
            Command::Down(units) => self.aim += units,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run11() {
        assert_eq!(
            run1("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2"),
            150
        );
    }

    #[test]
    fn run21() {
        assert_eq!(
            run2("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2"),
            900
        );
    }
}
