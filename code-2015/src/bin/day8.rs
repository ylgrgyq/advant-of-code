use std::{error, io, str::Chars};

fn main() -> io::Result<()> {
    match io::stdin().lines().collect::<Result<Vec<String>, _>>() {
        Ok(lines) => {
            let original: usize = lines.iter().map(|s| s.len()).sum();
            let memory: usize = lines
                .iter()
                .map(|s| count_characters_in_memory(s.as_str()).unwrap())
                .sum();
            println!("num of orignal characters is: {}", original);
            println!("num of characters in memory is: {}", memory);
            println!(
                "difference from memory to original is: {}",
                original - memory
            );

            let encoded: usize = lines.iter().map(|s| encode(s.as_str()).len()).sum();
            println!("num of encoded characters is: {}", encoded);
            println!(
                "difference from encoded to original is: {}",
                encoded - original
            );
        }
        Err(e) => println!("parse instructions failed. {}", e),
    }
    Ok(())
}

fn count_characters_in_memory(s: &str) -> Result<usize, Box<dyn error::Error>> {
    if !s.starts_with("\"") || !s.ends_with("\"") {
        return Err(format!("string: \"{}\" is not quoted by double quote", s).into());
    }

    fn count_escape(chars: &mut Chars, c: &char) -> Result<usize, Box<dyn error::Error>> {
        match c {
            '\\' => Ok(chars
                .next()
                .and_then(|nc| match nc {
                    'x' => chars
                        .next()
                        .and_then(|_| chars.next().and_then(|_| Some(1)).or(Some(3)))
                        .or(Some(2)),
                    '\\' => Some(1),
                    '\"' => Some(1),
                    _ => Some(2),
                })
                .or(Some(1))
                .unwrap()),
            '\"' => Err("found multiple double quote".into()),

            _ => Ok(1),
        }
    }

    let mut count_chs = 0;
    let mut chars = s[1..s.len() - 1].chars();
    loop {
        match chars.next() {
            Some(c) => match count_escape(&mut chars, &c) {
                Ok(num) => count_chs += num,
                Err(e) => return Err(format!("got error: \"{}\" for string: \"{}\"", e, s).into()),
            },
            None => return Ok(count_chs),
        }
    }
}

fn encode(s: &str) -> String {
    return format!(
        "\"{}\"",
        s.chars()
            .map(|c| match c {
                '\"' => String::from("\\\""),
                '\\' => String::from("\\\\"),
                _ => format!("{}", c),
            })
            .collect::<Vec<String>>()
            .concat()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_characters() {
        assert_eq!("\"\"".len(), 2);
        assert_eq!("\"abc\"".len(), 5);
        assert_eq!("\"aaa\\\"aaa\"".len(), 10);
        assert_eq!("\"\\x27\"".len(), 6);
    }
    #[test]
    fn test_count_characters_in_memory() {
        assert_eq!(count_characters_in_memory("\"\\\"").unwrap(), 1);
        assert_eq!(count_characters_in_memory("\"\\\\\"").unwrap(), 1);
        assert_eq!(count_characters_in_memory("\"\\\"\"").unwrap(), 1);
        assert_eq!(count_characters_in_memory("\"\\x\"").unwrap(), 2);
        assert_eq!(count_characters_in_memory("\"\\x1\"").unwrap(), 3);
        assert_eq!(count_characters_in_memory("\"\\x12\"").unwrap(), 1);

        assert_eq!(count_characters_in_memory("\"abc\"").unwrap(), 3);
        assert_eq!(count_characters_in_memory("\"aaa\\\"aaa\"").unwrap(), 7);
        assert_eq!(count_characters_in_memory("\"\x27\"").unwrap(), 1);
    }

    #[test]
    fn test_encode() {
        println!("asdfsdf {}", encode(""));
        assert_eq!(encode("\"\"").len(), 6);
        assert_eq!(encode("\"abc\"").len(), 9);
        assert_eq!(encode("\"aaa\\\"aaa\"").len(), 16);
        assert_eq!(encode("\"\\x27\"").len(), 11);
    }
}
