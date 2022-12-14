use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let ret = sum_all_numbers(buffer.as_str());
    println!("result is {}", ret);
    Ok(())
}

fn sum_all_numbers(s: &str) -> i32 {
    return get_number(s)
        .iter()
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .sum();
}

fn get_number(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut tmp = String::new();
    for c in s.chars() {
        if c.is_digit(10) || c == '-' {
            tmp.push(c);
            continue;
        }
        if !tmp.is_empty() {
            out.push(tmp);
            tmp = String::new();
        }
    }
    if !tmp.is_empty() {
        out.push(tmp);
    }
    println!("Sd {:?}", out);
    return out;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_all_numbers() {
        assert_eq!(sum_all_numbers("[1,2,3]"), 6);
        assert_eq!(sum_all_numbers("{\"a\":2,\"b\":4}"), 6);
        assert_eq!(sum_all_numbers("[[[3]]"), 3);
        assert_eq!(sum_all_numbers("{\"a\":{\"b\":4},\"c\":-1}"), 3);
        assert_eq!(sum_all_numbers("{\"a\":[-1,1]}"), 0);
        assert_eq!(sum_all_numbers("[-1,{\"a\":1}]"), 0);
        assert_eq!(sum_all_numbers("{}"), 0);
        assert_eq!(sum_all_numbers("[]"), 0);
    }
}
