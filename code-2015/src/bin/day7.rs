use std::{collections::HashMap, error, io, str::FromStr};

fn main() -> io::Result<()> {
    match io::stdin()
        .lines()
        .collect::<Result<Vec<String>, _>>()?
        .iter()
        .map(|l| l.parse::<Sentence>())
        .collect::<Result<Vec<Sentence>, Box<dyn error::Error>>>()
    {
        Ok(sentences) => {
            let mut circuit = Circuit::new();
            circuit.execute(sentences);
            let part_one_value = circuit.get_value(&"a".into()).unwrap();
            println!("part one value for wire a is {}", part_one_value);

            let sorted = circuit.sorted;
            circuit = Circuit::new();
            circuit.put_value(&"b".into(), part_one_value);
            circuit.execute(sorted);
            let part_two_value = circuit.get_value(&"a".into()).unwrap();
            println!("part two value for wire a is {}", part_two_value);
        }
        Err(e) => println!("parse instructions failed. {}", e),
    }
    Ok(())
}

#[derive(Debug)]
enum Operator {
    ASSIGN(String),
    NOT(String),
    AND { x: String, y: String },
    OR { x: String, y: String },
    LSHIFT { x: String, y: String },
    RSHIFT { x: String, y: String },
}

impl FromStr for Operator {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splited = s.split_whitespace().collect::<Vec<&str>>();
        return match splited.len() {
            1 => Some(Operator::ASSIGN(splited[0].into())),
            2 => {
                if "NOT".eq(splited[0]) {
                    Some(Operator::NOT(splited[1].into()))
                } else {
                    None
                }
            }
            3 => {
                let x: String = splited[0].into();
                let y: String = splited[2].into();
                match splited[1] {
                    "AND" => Some(Operator::AND { x, y }),
                    "OR" => Some(Operator::OR { x, y }),
                    "LSHIFT" => Some(Operator::LSHIFT { x, y }),
                    "RSHIFT" => Some(Operator::RSHIFT { x, y }),
                    _ => None,
                }
            }
            _ => None,
        }
        .ok_or(format!("can't parse operator form sentence: \"{}\"", s).into());
    }
}

impl Operator {
    fn execute(&self, circuit: &Circuit) -> Option<u16> {
        match self {
            Self::ASSIGN(x) => self.get_or_parse_operand(circuit, x),
            Self::NOT(x) => self.get_or_parse_operand(circuit, x).map(|v| !v),
            Self::AND { x, y } => self.get_or_parse_operand(circuit, x).and_then(|xv| {
                self.get_or_parse_operand(circuit, y)
                    .and_then(|yv| Some(xv & yv))
            }),
            Self::OR { x, y } => self.get_or_parse_operand(circuit, x).and_then(|xv| {
                self.get_or_parse_operand(circuit, y)
                    .and_then(|yv| Some(xv | yv))
            }),
            Self::LSHIFT { x, y } => self.get_or_parse_operand(circuit, x).and_then(|xv| {
                self.get_or_parse_operand(circuit, y)
                    .and_then(|yv| Some(xv << yv))
            }),
            Self::RSHIFT { x, y } => self.get_or_parse_operand(circuit, x).and_then(|xv| {
                self.get_or_parse_operand(circuit, y)
                    .and_then(|yv| Some(xv >> yv))
            }),
        }
    }

    fn get_or_parse_operand(&self, circuit: &Circuit, operand: &String) -> Option<u16> {
        circuit.get_value(operand).or(operand.parse::<u16>().ok())
    }
}

#[derive(Debug)]
struct Sentence {
    operator: Operator,
    assign_to: String,
}

impl FromStr for Sentence {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return regex::Regex::new(r"(.*) -> ([a-z]+)$")
            .ok()
            .and_then(|re: regex::Regex| -> Option<Sentence> {
                return re.captures(s).and_then(|c| {
                    if c.len() < 3 {
                        return None;
                    }
                    let assign = String::from(&c[2]);
                    let operator = c[1].parse::<Operator>();
                    return operator.ok().and_then(|op| {
                        Some(Sentence {
                            assign_to: assign,
                            operator: op,
                        })
                    });
                });
            })
            .ok_or(format!("sentence: \"{}\" is not in valid format", s).into());
    }
}

impl Sentence {
    fn execute(&self, circuit: &mut Circuit) -> Option<()> {
        self.operator
            .execute(circuit)
            .map(|v| circuit.put_value(&self.assign_to, v))
    }
}
struct Circuit {
    values: HashMap<String, u16>,
    sorted: Vec<Sentence>,
}

impl Circuit {
    fn new() -> Circuit {
        return Circuit {
            sorted: vec![],
            values: HashMap::new(),
        };
    }

    fn get_value(&self, identifier: &String) -> Option<u16> {
        return self.values.get(identifier).map(|v| v.clone());
    }

    fn put_value(&mut self, identifier: &String, v: u16) {
        if !self.values.contains_key(identifier) {
            self.values.insert(identifier.clone(), v);
        }
    }

    fn execute(&mut self, mut sentences: Vec<Sentence>) {
        loop {
            if sentences.is_empty() {
                return;
            }

            sentences = sentences
                .into_iter()
                .filter_map(|s| match &s.execute(self) {
                    Some(_) => {
                        self.sorted.push(s);
                        None
                    }
                    None => Some(s),
                })
                .collect::<Vec<Sentence>>();
        }
    }

    fn println_sentences(&self) {
        for s in self.sorted.iter() {
            println!("s {:?}", s)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_circuit() {
        let ses = vec![
            "x AND y -> d",
            "x OR y -> e",
            "123 -> x",
            "x LSHIFT 2 -> f",
            "y RSHIFT 2 -> g",
            "456 -> y",
            "NOT x -> h",
            "NOT y -> i",
        ]
        .into_iter()
        .map(|s| s.parse::<Sentence>().unwrap())
        .collect::<Vec<Sentence>>();
        let mut c = Circuit::new();
        c.execute(ses);

        assert_eq!(c.get_value(&"x".into()).unwrap(), 123);
        assert_eq!(c.get_value(&"y".into()).unwrap(), 456);
        assert_eq!(c.get_value(&"d".into()).unwrap(), 72);
        assert_eq!(c.get_value(&"e".into()).unwrap(), 507);
        assert_eq!(c.get_value(&"f".into()).unwrap(), 492);
        assert_eq!(c.get_value(&"g".into()).unwrap(), 114);
        assert_eq!(c.get_value(&"h".into()).unwrap(), 65412);
        assert_eq!(c.get_value(&"i".into()).unwrap(), 65079);
    }
}
