use std::{char::from_u32_unchecked, io};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let mut pass = generate_next_valid_password(buffer);
    println!("next valid password is: {}", pass);
    pass = generate_next_valid_password(pass);
    println!("next next valid password is: {}", pass);
    Ok(())
}

fn generate_next_valid_password(s: String) -> String {
    let mut pass = s;
    loop {
        pass = generate_next_password(&pass);
        if is_valid(&pass) {
            return pass;
        }
    }
}

fn generate_next_password(s: &String) -> String {
    let mut out = String::new();
    let mut pass = false;
    for c in s.chars().rev() {
        if pass {
            out.push(c);
            continue;
        }
        let char_num = c as u32;
        if char_num + 1 <= 'z' as u32 {
            unsafe {
                out.push(from_u32_unchecked(char_num + 1));
            }
            pass = true;
        } else {
            out.push('a');
        }
    }
    return out.chars().rev().collect();
}

fn is_valid(s: &String) -> bool {
    let mut increasing_chars: Vec<u32> = vec![];
    let mut pairs = vec![];
    let mut last_char_opt = None;
    for c in s.chars() {
        if c == 'i' || c == 'o' || c == 'l' {
            return false;
        }

        let char_num = c as u32;
        if increasing_chars.len() < 3 {
            if increasing_chars
                .last()
                .and_then(|v| Some(char_num == (*v + 1)))
                .unwrap_or(false)
            {
                increasing_chars.push(char_num);
                last_char_opt = Some(c);
                continue;
            }
            increasing_chars.clear();
            increasing_chars.push(char_num);
        }

        if last_char_opt
            .and_then(|last_c| Some(last_c == c))
            .unwrap_or(false)
        {
            pairs.push(c);
            last_char_opt = None;
            continue;
        }
        last_char_opt = Some(c);
    }
    return increasing_chars.len() == 3 && pairs.len() >= 2;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid(&"hijklmmn".into()), false);
        assert_eq!(is_valid(&"abbceffg".into()), false);
        assert_eq!(is_valid(&"abbcegjk".into()), false);
        assert_eq!(is_valid(&"abcdffaa".into()), true);
        assert_eq!(is_valid(&"ghjaabcc".into()), true);
    }

    #[test]
    fn test_generate_password() {
        assert_eq!(generate_next_password(&"hijklmmn".into()), "hijklmmo");
        assert_eq!(generate_next_password(&"hijklmmz".into()), "hijklmna");
        assert_eq!(generate_next_password(&"hizzzzzz".into()), "hjaaaaaa");
    }
}
