use std::{io, num::ParseIntError, str::FromStr};

fn main() -> io::Result<()> {
    let lines: Result<Vec<String>, _> = io::stdin().lines().collect();
    match lines {
        Ok(input) => {
            let lines_str = &input.iter().map(|l| l.as_str()).collect();
            match calculate_total_wrappers(lines_str) {
                Ok(t) => println!("total square feet of wrapping paper: {}", t),
                Err(e) => match e {
                    CalculateError::InputFormatError(input) => {
                        println!("invalid format for input string: \"{}\"", input)
                    }
                    CalculateError::ParseStringError(input, _) => {
                        println!("parse number from input string: \"{}\" failed", input)
                    }
                },
            };

            match calculate_total_ribbons(lines_str) {
                Ok(t) => println!("total ribbon: {}", t),
                Err(e) => match e {
                    CalculateError::InputFormatError(input) => {
                        println!("invalid format for input string: \"{}\"", input)
                    }
                    CalculateError::ParseStringError(input, _) => {
                        println!("parse number from input string: \"{}\" failed", input)
                    }
                },
            };
        }
        Err(e) => println!("read input lines faied, {}", e),
    }

    Ok(())
}

struct Rectangular {
    length: usize,
    width: usize,
    height: usize,
}

impl Rectangular {
    fn calculate_wrapping_paper(&self) -> usize {
        let a = self.length * self.width;
        let b = self.length * self.height;
        let c = self.width * self.height;
        let r = c.min(a.min(b));
        return 2 * (a + b + c) + r;
    }

    fn calculate_ribbon(&self) -> usize {
        let ribbon = self.length * self.height * self.width;
        let mut edges = vec![self.length, self.height, self.width];
        edges.sort();
        edges.reverse();
        let r = edges[1..]
            .into_iter()
            .map(|e| *e)
            .reduce(|total: usize, next: usize| -> usize { return total + next })
            .unwrap_or(0);
        return ribbon + 2 * r;
    }
}

impl FromStr for Rectangular {
    type Err = CalculateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s.split("x").collect();
        if splitted.len() != 3 {
            return Err(CalculateError::InputFormatError(String::from(s)));
        }
        return splitted[0]
            .parse::<usize>()
            .and_then(|length| {
                splitted[1].parse::<usize>().and_then(|width| {
                    splitted[2].parse::<usize>().and_then(|height| {
                        return Ok(Rectangular {
                            length,
                            width,
                            height,
                        });
                    })
                })
            })
            .or_else(|e| return Err(CalculateError::ParseStringError(String::from(s), e)));
    }
}

#[derive(Debug, PartialEq)]
enum CalculateError {
    InputFormatError(String),
    ParseStringError(String, ParseIntError),
}

fn calculate_total_wrappers(input: &Vec<&str>) -> Result<usize, CalculateError> {
    let mut total_wrapper: usize = 0;
    for line in input {
        let rec = Rectangular::from_str(line)?;
        total_wrapper += rec.calculate_wrapping_paper();
    }
    return Ok(total_wrapper);
}

fn calculate_total_ribbons(input: &Vec<&str>) -> Result<usize, CalculateError> {
    let mut total: usize = 0;
    for line in input {
        let rec = Rectangular::from_str(line)?;
        total += rec.calculate_ribbon();
    }
    return Ok(total);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_invalid_input() {
        let mut input = "";
        assert_eq!(
            calculate_total_wrappers(&vec![input]).unwrap_err(),
            CalculateError::InputFormatError(String::from(input))
        );
        input = "1x";
        assert_eq!(
            calculate_total_wrappers(&vec![input]).unwrap_err(),
            CalculateError::InputFormatError(String::from(input))
        );
        input = "1x1x1x1";
        assert_eq!(
            calculate_total_wrappers(&vec![input]).unwrap_err(),
            CalculateError::InputFormatError(String::from(input))
        );
    }
    #[test]
    fn test_calculate_wrappers() {
        assert_eq!(calculate_total_wrappers(&vec!["2x3x4"]).unwrap(), 58);
        assert_eq!(calculate_total_wrappers(&vec!["1x1x10"]).unwrap(), 43);
    }
    #[test]
    fn test_calculate_ribbon() {
        assert_eq!(calculate_total_ribbons(&vec!["2x3x4"]).unwrap(), 34);
        assert_eq!(calculate_total_ribbons(&vec!["1x1x10"]).unwrap(), 14);
    }
}
