use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let read_ret = io::stdin().read_line(&mut buffer);
    match read_ret {
        Ok(_) => {
            let ret = check_md5(buffer.as_str(), 5);
            println!("coin with 5 leading zero is: {}", ret);
            let ret = check_md5(buffer.as_str(), 6);
            println!("coin with 6 leading zero is: {}", ret);
            return Ok(());
        }
        Err(err) => {
            println!("invalid input string: \"{}\"", err);
            return Err(err);
        }
    }
}

fn check_md5(input: &str, num_of_zeros: usize) -> u64 {
    let mut i = 0;
    loop {
        let leading_zeros = "0".repeat(num_of_zeros);
        let k = format!("{}{}", input.trim(), i);
        let digest = format!("{:x}", md5::compute(k.as_bytes()));
        if digest.starts_with(leading_zeros.as_str()) {
            return i;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_string() {
        assert_eq!(check_md5("abcdef", 5), 609043);
        assert_eq!(check_md5("pqrstuv", 5), 1048970);
    }
}
