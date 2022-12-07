use std::{fmt, io, str::FromStr};

fn main() -> io::Result<()> {
    match io::stdin()
        .lines()
        .collect::<Result<Vec<String>, _>>()?
        .iter()
        .map(|l| l.parse::<Instruction>())
        .collect::<Result<Vec<Instruction>, ParseInstructionError>>()
    {
        Ok(ins) => {
            {
                let mut grid = Grid::new(1000);
                ins.iter().for_each(|i| {
                    grid.apply(i);
                });
                println!("{} lights are lit", grid.count());
            }

            {
                let mut grid = Grid::new(1000);
                ins.iter().for_each(|i| {
                    grid.apply_brightness(i);
                });
                println!("total brightness is {}", grid.count());
            }
        }
        Err(e) => println!("parse instructions failed. {}", e),
    }
    Ok(())
}

#[derive(Debug)]
enum Action {
    TurnOn,
    TurnOff,
    TOGGLE,
}

impl FromStr for Action {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "turn on" => return Ok(Action::TurnOn),
            "turn off" => return Ok(Action::TurnOff),
            "toggle" => return Ok(Action::TOGGLE),
            _ => Err(ParseInstructionError),
        }
    }
}

#[derive(Debug)]
struct Position(usize, usize);

impl FromStr for Position {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splited = s
            .split(",")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()
            .map_err(|_| ParseInstructionError)?;
        if splited.len() < 2 {
            return Err(ParseInstructionError);
        }
        return Ok(Position(splited[0], splited[1]));
    }
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    start: Position,
    end: Position,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseInstructionError;

impl fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "provided string was not `true` or `false`".fmt(f)
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return regex::Regex::new(r"^(turn on|turn off|toggle) (\d+,\d+) through (\d+,\d+)$")
            .map_err(|_| ParseInstructionError)
            .and_then(
                |re: regex::Regex| -> Result<Instruction, ParseInstructionError> {
                    return re
                        .captures(s)
                        .map(|c| {
                            if c.len() < 4 {
                                return None;
                            }
                            return c[1]
                                .parse::<Action>()
                                .and_then(|action| {
                                    c[2].parse::<Position>().and_then(|start| {
                                        c[3].parse::<Position>()
                                            .and_then(|end| Ok(Instruction { action, start, end }))
                                    })
                                })
                                .ok();
                        })
                        .unwrap_or(None)
                        .map_or(Err(ParseInstructionError), |i| Ok(i));
                },
            );
    }
}

struct Grid {
    grid: Vec<Vec<u32>>,
    size: usize,
}

impl Grid {
    fn new(size: usize) -> Grid {
        let grid = vec![vec![0; size]; size];
        return Grid { grid, size };
    }

    fn count(&self) -> u32 {
        let mut count: u32 = 0;
        for i in 0..self.size {
            for j in 0..self.size {
                count += self.grid[i][j];
            }
        }
        return count;
    }
    fn apply(&mut self, instruction: &Instruction) {
        for i in instruction.start.1..=instruction.end.1 {
            for j in instruction.start.0..=instruction.end.0 {
                self.grid[i][j] = match instruction.action {
                    Action::TurnOn => 1,
                    Action::TurnOff => 0,
                    Action::TOGGLE => match self.grid[i][j] {
                        0 => 1,
                        _ => 0,
                    },
                }
            }
        }
    }
    fn apply_brightness(&mut self, instruction: &Instruction) {
        for i in instruction.start.1..=instruction.end.1 {
            for j in instruction.start.0..=instruction.end.0 {
                match instruction.action {
                    Action::TurnOn => self.grid[i][j] += 1,
                    Action::TurnOff => {
                        if self.grid[i][j] > 0 {
                            self.grid[i][j] -= 1
                        }
                    }
                    Action::TOGGLE => self.grid[i][j] += 2,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_grid() {
        assert_eq!(
            execute_grid_instruction("turn on 0,0 through 999,999", 1000),
            1000000
        );
        assert_eq!(
            execute_grid_instruction("toggle 0,0 through 999,0", 1000),
            1000
        );
        assert_eq!(
            execute_grid_instruction("turn on 499,499 through 500,500", 1000),
            4
        );
        assert_eq!(execute_grid_instruction("turn on 0,0 through 1,1", 4), 4);
    }
    #[test]
    fn test_bright_grid() {
        assert_eq!(
            execute_bright_grid_instruction("toggle 0,0 through 999,999", 1000),
            2000000
        );
        assert_eq!(
            execute_bright_grid_instruction("turn on 0,0 through 0,0", 1000),
            1
        );
        assert_eq!(
            execute_bright_grid_instruction("turn on 0,0 through 1,1", 4),
            4
        );
        assert_eq!(
            execute_bright_grid_instruction("toggle 0,0 through 1,1", 4),
            8
        );
    }

    fn execute_grid_instruction(s: &str, size: usize) -> u32 {
        let mut g = Grid::new(size);
        g.apply(&s.parse::<Instruction>().unwrap());
        return g.count();
    }

    fn execute_bright_grid_instruction(s: &str, size: usize) -> u32 {
        let mut g = Grid::new(size);
        g.apply_brightness(&s.parse::<Instruction>().unwrap());
        return g.count();
    }
}
