use std::{error, io};

use serde_json::Value;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let ret = sum_all_numbers(buffer.as_str());
    println!("result is {}", ret);

    let ret2 = sum_without_red(buffer.as_str()).unwrap();
    println!("result is {}", ret2);
    Ok(())
}

fn sum_all_numbers(s: &str) -> i32 {
    SearchNumberJsonDocument { input: s.into() }
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .sum()
}

fn sum_without_red(s: &str) -> Result<i64, Box<dyn error::Error>> {
    let v: Value = serde_json::from_str(s)?;
    return Ok(sum_number(&v));
}

struct SearchNumberJsonDocument {
    input: String,
}

impl Iterator for SearchNumberJsonDocument {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        return self
            .input
            .chars()
            .position(|c| c.is_digit(10) || c == '-')
            .and_then(|i| {
                let out: String = self.input[i..]
                    .chars()
                    .take_while(|c| c.is_digit(10) || c.eq(&'-'))
                    .collect();
                self.input = self.input[(i + out.len())..].into();
                Some(out)
            });
    }
}

fn sum_number(v: &Value) -> i64 {
    match v {
        Value::Array(arr) => arr.iter().map(|val| sum_number(val)).sum::<i64>(),
        Value::Object(obj) => {
            if has_red(&mut obj.values()) {
                return 0;
            }
            obj.values().map(|val| sum_number(val)).sum::<i64>()
        }
        Value::Number(n) => n.as_i64().unwrap(),
        _ => 0,
    }
}

fn has_red(iter: &mut dyn Iterator<Item = &Value>) -> bool {
    iter.filter_map(|val| {
        if val.is_string() {
            return Some(val.as_str().unwrap());
        }
        return None;
    })
    .find(|val| val.eq(&"red"))
    .is_some()
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

    #[test]
    fn test_sum_without_red() {
        assert_eq!(sum_without_red("[1,2,3]").unwrap(), 6);
        assert_eq!(sum_without_red("[1,{\"c\":\"red\",\"b\":2},3]").unwrap(), 4);
        assert_eq!(
            sum_without_red("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}").unwrap(),
            0
        );
        assert_eq!(sum_without_red("[1,\"red\",5]").unwrap(), 6);
    }
}
