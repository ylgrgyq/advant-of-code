use std::{collections::HashSet, io};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let read_ret = io::stdin().read_line(&mut buffer);
    match read_ret {
        Ok(_) => {
            let ret = spread_presents(buffer.as_str());
            println!("Santa visited {} houses", ret);
            let ret = spread_presents_with_robot(buffer.as_str());
            println!("Santa and Robot-Santa visited {} houses", ret);
            return Ok(());
        }
        Err(err) => {
            println!("invalid input string: \"{}\"", err);
            return Err(err);
        }
    }
}

fn spread_presents(input: &str) -> usize {
    if input.is_empty() {
        return 0;
    }

    let mut pos = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(pos);
    for direct in input.chars() {
        pos = next_pos(direct, pos);
        visited.insert(pos);
    }
    return visited.len();
}

fn spread_presents_with_robot(input: &str) -> usize {
    if input.is_empty() {
        return 0;
    }

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));
    for actor in 0..2 {
        let mut pos = (0, 0);
        for direct in input.chars().skip(actor).step_by(2) {
            pos = next_pos(direct, pos);
            visited.insert(pos);
        }
    }

    return visited.len();
}

fn next_pos(direction: char, pos: (i32, i32)) -> (i32, i32) {
    let mut new_x = pos.0;
    let mut new_y = pos.1;
    match direction {
        '^' => new_y += 1,
        '>' => new_x += 1,
        'v' => new_y -= 1,
        '<' => new_x -= 1,
        _ => (),
    };
    return (new_x, new_y);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_string() {
        assert_eq!(spread_presents(""), 0);
    }
    #[test]
    fn test_spread_presents() {
        assert_eq!(spread_presents(">"), 2);
        assert_eq!(spread_presents("^>v<"), 4);
        assert_eq!(spread_presents("^v^v^v^v^v"), 2);
        assert_eq!(spread_presents("<><><><>"), 2);
    }
    #[test]
    fn test_spread_presents_with_robot() {
        assert_eq!(spread_presents_with_robot("^v"), 3);
        assert_eq!(spread_presents_with_robot("^>v<"), 3);
        assert_eq!(spread_presents_with_robot("^v^v^v^v^v"), 11);
        assert_eq!(spread_presents_with_robot("<><><><>"), 9);
    }
}
