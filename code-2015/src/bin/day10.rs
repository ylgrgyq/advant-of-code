use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let ret = look_and_say(buffer.trim(), 40);
    println!("length of the result is: {}", ret.len());

    let ret2 = look_and_say(ret.as_str(), 10);
    println!("length of the result is: {}", ret2.len());
    Ok(())
}

fn look_and_say(s: &str, repeat_times: u16) -> String {
    let mut last = s.into();
    for _ in 0..repeat_times {
        last = do_look_and_say(last);
    }
    return last;
}

fn do_look_and_say(s: String) -> String {
    let mut out = String::new();
    let mut last = None;
    let mut num = 0;
    for c in s.chars() {
        match last {
            Some(last_c) => {
                if last_c == c {
                    num += 1;
                } else {
                    out.push_str(format!("{}{}", num, last_c).as_str());
                    last = Some(c);
                    num = 1;
                }
            }
            None => {
                last = Some(c);
                num += 1;
            }
        }
    }
    if num > 0 {
        out.push_str(format!("{}{}", num, last.unwrap()).as_str());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_do_look_and_say() {
        assert_eq!(do_look_and_say("".into()), "");
        assert_eq!(do_look_and_say("1".into()), "11");
        assert_eq!(do_look_and_say("11".into()), "21");
        assert_eq!(do_look_and_say("21".into()), "1211");
        assert_eq!(do_look_and_say("1211".into()), "111221");
        assert_eq!(do_look_and_say("111221".into()), "312211");
    }
    #[test]
    fn test_look_and_say() {
        assert_eq!(look_and_say("1", 5), "312211");
        assert_eq!(look_and_say("1321131112", 1), "11131221133112");
    }
}
