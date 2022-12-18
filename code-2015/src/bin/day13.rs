use std::{
    collections::{HashMap, HashSet},
    error, io,
    str::FromStr,
    vec,
};

fn main() -> io::Result<()> {
    match io::stdin()
        .lines()
        .collect::<Result<Vec<String>, _>>()?
        .iter()
        .map(|l| l.parse::<Survey>())
        .collect::<Result<Vec<Survey>, Box<dyn error::Error>>>()
    {
        Ok(surveys) => {
            let p = Puzzle::new(surveys);
            let (plan, happiness) = p.optimal_plan();
            println!("optimal happiness is {}, plan is: {:?}", happiness, plan);
            let (plan, happiness) = p.optimal_plan_with_me();
            println!(
                "optimal happiness with me is {}, plan is: {:?}",
                happiness, plan
            );
        }
        Err(e) => println!("parse instructions failed. {}", e),
    }
    Ok(())
}

struct Puzzle {
    survey_map: HashMap<String, HashMap<String, i32>>,
    guests: HashSet<String>,
    guests_and_me: HashSet<String>,
}

impl Puzzle {
    fn new(surveies: Vec<Survey>) -> Puzzle {
        let guests = surveies
            .iter()
            .map(|s| s.from.clone())
            .collect::<HashSet<String>>();
        let mut survey_map = potential_happiness_survey(surveies);
        survey_map = add_me_to_survey_map(survey_map, &guests);
        let mut guests_and_me = guests.clone();
        guests_and_me.insert("Me".into());
        Puzzle {
            survey_map,
            guests,
            guests_and_me,
        }
    }

    fn optimal_plan(&self) -> (Vec<&String>, i32) {
        let plans = permutations(self.guests.iter().collect::<Vec<&String>>());

        let max = plans.iter().map(|p| self.get_happiness(p)).max().unwrap();
        let plan = plans.iter().find(|p| self.get_happiness(p) == max).unwrap();
        return (plan.clone(), max);
    }

    fn optimal_plan_with_me(&self) -> (Vec<&String>, i32) {
        let plans = permutations(self.guests_and_me.iter().collect::<Vec<&String>>());

        let max = plans.iter().map(|p| self.get_happiness(p)).max().unwrap();
        let plan = plans.iter().find(|p| self.get_happiness(p) == max).unwrap();
        return (plan.clone(), max);
    }

    fn get_happiness(&self, plan: &Vec<&String>) -> i32 {
        if plan.len() < 2 {
            return 0;
        }

        let mut happiness = 0;
        for i in 0..plan.len() {
            let mut next = i + 1;
            if next == plan.len() {
                next = 0;
            }
            happiness += self
                .survey_map
                .get(plan[i])
                .unwrap()
                .get(plan[next])
                .unwrap();
            happiness += self
                .survey_map
                .get(plan[next])
                .unwrap()
                .get(plan[i])
                .unwrap();
        }

        happiness
    }
}

fn potential_happiness_survey(surveies: Vec<Survey>) -> HashMap<String, HashMap<String, i32>> {
    let mut map: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for survey in surveies {
        if !map.contains_key(&survey.from) {
            map.insert(survey.from.clone(), HashMap::new());
        }
        let val_map = map.get_mut(&survey.from).unwrap();
        val_map.insert(survey.to, survey.happiness);
    }
    map
}

fn permutations(input: Vec<&String>) -> Vec<Vec<&String>> {
    if input.is_empty() {
        return vec![];
    }
    if input.len() == 1 {
        return vec![input];
    }

    let elem = input[0];
    let sub_permutations = permutations(input[1..].to_vec());
    let mut out: Vec<Vec<&String>> = vec![];
    for sub in sub_permutations {
        for i in 0..sub.len() + 1 {
            let mut v = sub.clone();
            v.insert(i, elem);
            out.push(v);
        }
    }
    out
}

fn add_me_to_survey_map(
    mut survey_map: HashMap<String, HashMap<String, i32>>,
    guests: &HashSet<String>,
) -> HashMap<String, HashMap<String, i32>> {
    for val_map in survey_map.values_mut() {
        val_map.insert("Me".into(), 0);
    }
    let mut me_map = HashMap::new();
    for guest in guests {
        me_map.insert(guest.clone(), 0);
    }
    survey_map.insert("Me".into(), me_map);
    survey_map
}

#[derive(Debug)]
struct Survey {
    from: String,
    to: String,
    happiness: i32,
}

impl FromStr for Survey {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return regex::Regex::new(
            r"(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+).$",
        )
        .ok()
        .and_then(|re: regex::Regex| -> Option<Survey> {
            return re.captures(s).and_then(|c| {
                if c.len() < 5 {
                    return None;
                }
                let from = String::from(&c[1]);
                let to = String::from(&c[4]);
                let happiness = c[3].parse::<i32>();
                return happiness.ok().and_then(|mut h| {
                    if c[2].eq("lose") {
                        h = -1 * h;
                    }
                    Some(Survey {
                        from,
                        to,
                        happiness: h,
                    })
                });
            });
        })
        .ok_or(format!("puzzle input: \"{}\" is not in valid format", s).into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_optimal_plan() {
        let surveys = vec![
            "Alice would gain 54 happiness units by sitting next to Bob.",
            "Alice would lose 79 happiness units by sitting next to Carol.",
            "Alice would lose 2 happiness units by sitting next to David.",
            "Bob would gain 83 happiness units by sitting next to Alice.",
            "Bob would lose 7 happiness units by sitting next to Carol.",
            "Bob would lose 63 happiness units by sitting next to David.",
            "Carol would lose 62 happiness units by sitting next to Alice.",
            "Carol would gain 60 happiness units by sitting next to Bob.",
            "Carol would gain 55 happiness units by sitting next to David.",
            "David would gain 46 happiness units by sitting next to Alice.",
            "David would lose 7 happiness units by sitting next to Bob.",
            "David would gain 41 happiness units by sitting next to Carol.",
        ]
        .iter()
        .map(|s| s.parse::<Survey>().unwrap())
        .collect();
        let p = Puzzle::new(surveys);
        let (plan, happiness) = p.optimal_plan();
        println!("plan {:?} {}", plan, happiness);
        assert_eq!(happiness, 330);
    }
}
