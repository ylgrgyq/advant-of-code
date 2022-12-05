use std::io;

fn main() -> io::Result<()> {
    let lines: Result<Vec<String>, _> = io::stdin().lines().collect();
    match lines {
        Ok(input) => {
            let mut num = input
                .iter()
                .filter(|l| check_nice_string(l.as_str()))
                .count();
            println!("Has {} nice strings", num);

            num = input
                .iter()
                .filter(|l| check_new_nice_string(l.as_str()))
                .count();
            println!("Has new {} nice strings", num);
        }
        Err(e) => println!("read input lines faied, {}", e),
    }
    Ok(())
}

fn check_nice_string(input: &str) -> bool {
    if input
        .chars()
        .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
        < 3
    {
        return false;
    }

    for bad_str in ["ab", "cd", "pq", "xy"] {
        if input.contains(bad_str) {
            return false;
        }
    }

    for r in input.chars().zip(input.chars().skip(1)) {
        if r.0 == r.1 {
            return true;
        }
    }

    return false;
}

fn check_new_nice_string(input: &str) -> bool {
    if has_repeat_char(input) {
        return repeat_without_overlap(input);
    }

    return false;
}

fn repeat_without_overlap(input: &str) -> bool {
    for i in 0..input.len() - 3 {
        let seed = &input[i..i + 2];
        if input[i + 2..].contains(seed) {
            return true;
        }
    }
    return false;
}

fn has_repeat_char(input: &str) -> bool {
    for r in input.chars().zip(input.chars().skip(2)) {
        if r.0 == r.1 {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_nice_string() {
        assert_eq!(check_nice_string("ugknbfddgicrmopn"), true);
        assert_eq!(check_nice_string("aaa"), true);
        assert_eq!(check_nice_string("jchzalrnumimnmhp"), false);
        assert_eq!(check_nice_string("haegwjzuvuyypxyu"), false);
        assert_eq!(check_nice_string("dvszwmarrgswjxmb"), false);
    }
    #[test]
    fn test_check_new_nice_string() {
        assert_eq!(check_new_nice_string("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(check_new_nice_string("xxyxx"), true);
        assert_eq!(check_new_nice_string("uurcxstgmygtbstg"), false);
        assert_eq!(check_new_nice_string("ieodomkazucvgmuy"), false);
    }
}
