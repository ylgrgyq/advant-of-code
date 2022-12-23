use std::{collections::HashMap, error, io, str::FromStr, vec};

fn main() -> io::Result<()> {
    match io::stdin()
        .lines()
        .collect::<Result<Vec<String>, _>>()?
        .iter()
        .map(|l| l.parse::<Ingredients>())
        .collect::<Result<Vec<Ingredients>, Box<dyn error::Error>>>()
    {
        Ok(ingredients) => {
            println!("highest score is {}", find_optimal(&ingredients, false));
            println!(
                "highest score is with calories equals to 500 is {}",
                find_optimal(&ingredients, true)
            );
        }
        Err(e) => println!("parse input failed. {}", e),
    }
    Ok(())
}

fn find_optimal(ingredients: &Vec<Ingredients>, filter_calories: bool) -> i64 {
    all_combinations(&ingredients, 100)
        .iter()
        .map(|props| get_score(ingredients, props, filter_calories))
        .max()
        .unwrap()
}

fn get_score(
    ingredients: &[Ingredients],
    props: &HashMap<&String, u32>,
    filter_calories: bool,
) -> i64 {
    let scores = vec![
        ingredients
            .iter()
            .map(|i| {
                props
                    .get(&i.name)
                    .and_then(|p| Some(*p as i64 * (i.capacity)))
                    .unwrap_or(0)
            })
            .sum::<i64>(),
        ingredients
            .iter()
            .map(|i| {
                props
                    .get(&i.name)
                    .and_then(|p| Some(*p as i64 * (i.durability)))
                    .unwrap_or(0)
            })
            .sum::<i64>(),
        ingredients
            .iter()
            .map(|i| {
                props
                    .get(&i.name)
                    .and_then(|p| Some(*p as i64 * (i.flavor)))
                    .unwrap_or(0)
            })
            .sum::<i64>(),
        ingredients
            .iter()
            .map(|i| {
                props
                    .get(&i.name)
                    .and_then(|p| Some(*p as i64 * (i.texture)))
                    .unwrap_or(0)
            })
            .sum::<i64>(),
    ];
    if filter_calories
        && ingredients
            .iter()
            .map(|i| {
                props
                    .get(&i.name)
                    .and_then(|p| Some(*p as i64 * (i.calories)))
                    .unwrap_or(0)
            })
            .sum::<i64>()
            != 500
    {
        return 0;
    }

    if scores.iter().any(|v| v < &0) {
        return 0;
    }

    return scores.iter().fold(1, |acc, item| acc * item);
}

fn all_combinations(ingredients: &[Ingredients], remain_amount: u32) -> Vec<HashMap<&String, u32>> {
    if ingredients.is_empty() || remain_amount <= 0 {
        return vec![];
    }
    if ingredients.len() < 2 {
        let mut props = HashMap::new();
        props.insert(&ingredients[0].name, remain_amount);
        return vec![props];
    }
    let mut out = vec![];
    out.append(&mut all_combinations(&ingredients[1..], remain_amount));

    for amount in 1..remain_amount {
        let m = all_combinations(&ingredients[1..], remain_amount - amount);
        for mut prop in m {
            prop.insert(&ingredients[0].name, amount);
            out.push(prop);
        }
    }
    let mut m = HashMap::new();
    m.insert(&ingredients[0].name, remain_amount);
    out.push(m);
    out
}

#[derive(Debug)]
struct Ingredients {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl FromStr for Ingredients {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return regex::Regex::new(
            r"(\w+): capacity ([\-0-9]+), durability ([\-0-9]+), flavor ([\-0-9]+), texture ([\-0-9]+), calories ([\-0-9]+)$",
        )
        .map_err(|_|"invalid input format".into())
        .and_then(|re| {
            let c = re.captures(s).unwrap();
            if c.len() < 7 {
                return Err("invalid input format".into());
            }
            let capacity = c[2].parse::<i64>().unwrap();
            let durability = c[3].parse::<i64>().unwrap();
            let flavor = c[4].parse::<i64>().unwrap();
            let texture = c[5].parse::<i64>().unwrap();
            let calories = c[6].parse::<i64>().unwrap();
            return Ok(Ingredients{name: c[1].into(), capacity, durability, flavor, texture, calories})
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_race() {
        let performances = vec![
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8",
            "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
        ]
        .iter()
        .map(|p| p.parse::<Ingredients>().unwrap())
        .collect::<Vec<Ingredients>>();
        assert_eq!(find_optimal(&performances, false), 62842880);
    }
}
