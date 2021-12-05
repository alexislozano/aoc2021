use crate::file;
use std::collections::HashMap;

pub fn run(part: u8) {
    let exo = 5;
    let input = file::read(exo);
    let result = match part {
        1 => run1(&input),
        _ => run2(&input),
    };
    file::write(exo, part, &result.to_string());
}

fn run1(input: &str) -> isize {
    let lines = parse(input);
    let mut overlaps: HashMap<(isize, isize), isize> = HashMap::new();
    for line in lines {
        for point in line.orthogonal_points() {
            if let Some(value) = overlaps.get_mut(&point) {
                *value += 1;
            } else {
                overlaps.insert(point, 1);
            }
        }
    }
    overlaps
        .into_values()
        .map(|value| if value > 1 { 1 } else { 0 })
        .sum()
}

fn run2(input: &str) -> isize {
    let lines = parse(input);
    let mut overlaps: HashMap<(isize, isize), isize> = HashMap::new();
    for line in lines {
        for point in line.all_points() {
            if let Some(value) = overlaps.get_mut(&point) {
                *value += 1;
            } else {
                overlaps.insert(point, 1);
            }
        }
    }
    overlaps
        .into_values()
        .map(|value| if value > 1 { 1 } else { 0 })
        .sum()
}

fn parse(input: &str) -> Vec<Line> {
    input
        .split("\n")
        .map(|line| {
            let split = line.split(" -> ").collect::<Vec<&str>>();
            let start = split[0].split(",").collect::<Vec<&str>>();
            let end = split[1].split(",").collect::<Vec<&str>>();
            Line::new(
                (
                    start[0].parse::<isize>().unwrap(),
                    start[1].parse::<isize>().unwrap(),
                ),
                (
                    end[0].parse::<isize>().unwrap(),
                    end[1].parse::<isize>().unwrap(),
                ),
            )
        })
        .collect::<Vec<Line>>()
}

struct Line {
    start: (isize, isize),
    end: (isize, isize),
}

impl Line {
    fn new(start: (isize, isize), end: (isize, isize)) -> Self {
        Self { start, end }
    }

    fn orthogonal_points(&self) -> Vec<(isize, isize)> {
        let (dx, dy) = if self.start.0 == self.end.0 {
            if self.start.1 < self.end.1 {
                (0, 1)
            } else {
                (0, -1)
            }
        } else if self.start.1 == self.end.1 {
            if self.start.0 < self.end.0 {
                (1, 0)
            } else {
                (-1, 0)
            }
        } else {
            return vec![];
        };
        let mut points = vec![self.start];
        let mut point = self.start;
        while point != self.end {
            point.0 += dx;
            point.1 += dy;
            points.push(point);
        }
        points
    }

    fn all_points(&self) -> Vec<(isize, isize)> {
        let (dx, dy) = if self.start.0 == self.end.0 {
            if self.start.1 < self.end.1 {
                (0, 1)
            } else {
                (0, -1)
            }
        } else if self.start.1 == self.end.1 {
            if self.start.0 < self.end.0 {
                (1, 0)
            } else {
                (-1, 0)
            }
        } else if self.end.1 - self.start.1 == self.end.0 - self.start.0 {
            if self.start.1 < self.end.1 {
                (1, 1)
            } else {
                (-1, -1)
            }
        } else if self.end.1 - self.start.1 == self.start.0 - self.end.0 {
            if self.start.1 < self.end.1 {
                (-1, 1)
            } else {
                (1, -1)
            }
        } else {
            return vec![];
        };
        let mut points = vec![self.start];
        let mut point = self.start;
        while point != self.end {
            point.0 += dx;
            point.1 += dy;
            points.push(point);
        }
        points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run11() {
        assert_eq!(run1("0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2"), 5);
    }

    #[test]
    fn run21() {
        assert_eq!(run2("0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2"), 12);
    }
}
