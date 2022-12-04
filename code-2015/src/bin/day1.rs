use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let read_ret = io::stdin().read_line(&mut buffer);
    match read_ret {
        Ok(_) => {
            let ret = check_result(buffer.as_str());
            println!(
                "Santa is at floor: {} and his first time to basement is at position: {}",
                ret.floor, ret.position
            );
            return Ok(());
        }
        Err(err) => {
            println!("invalid input string: \"{}\"", err);
            return Err(err);
        }
    }
}

#[derive(PartialEq, Debug)]
struct SantaResult {
    floor: i32,
    position: usize,
}

fn check_result(input: &str) -> SantaResult {
    if input.is_empty() {
        return SantaResult {
            floor: 0,
            position: 0,
        };
    }
    let mut floor = 0;
    let mut position: usize = 0;
    for (i, c) in input.chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if floor == -1 && position == 0 {
            position = i + 1;
        }
    }
    return SantaResult { floor, position };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_string() {
        assert_eq!(
            check_result(""),
            SantaResult {
                floor: 0,
                position: 0
            }
        );
    }
    #[test]
    fn test_floor() {
        assert_eq!(
            check_result("("),
            SantaResult {
                floor: 1,
                position: 0
            }
        );
        assert_eq!(
            check_result("()("),
            SantaResult {
                floor: 1,
                position: 0
            }
        );
        assert_eq!(
            check_result("()(("),
            SantaResult {
                floor: 2,
                position: 0
            }
        );
        assert_eq!(
            check_result("(())((("),
            SantaResult {
                floor: 3,
                position: 0
            }
        );
        assert_eq!(
            check_result("))((("),
            SantaResult {
                floor: 1,
                position: 1
            }
        );
    }
    #[test]
    fn test_position() {
        assert_eq!(
            check_result("(()))(("),
            SantaResult {
                floor: 1,
                position: 5
            }
        );
        assert_eq!(
            check_result(")"),
            SantaResult {
                floor: -1,
                position: 1
            }
        );
        assert_eq!(
            check_result("(()"),
            SantaResult {
                floor: 1,
                position: 0
            }
        );
    }
}
