use std::{collections::HashMap, error, io, str::FromStr};

fn main() -> io::Result<()> {
    match io::stdin()
        .lines()
        .collect::<Result<Vec<String>, _>>()?
        .iter()
        .map(|l| l.parse::<ReindeerPerformance>())
        .collect::<Result<Vec<ReindeerPerformance>, Box<dyn error::Error>>>()
    {
        Ok(performances) => {
            let race = ReindeerRace { performances };
            let ret = race.race(2503);
            println!("winning deer is {} and it's distance is {}", ret.0, ret.1);

            let ret2 = race.race_mode_two(2503);
            println!(
                "winning deer for race mode two is {} and it's points is {}",
                ret2.0, ret2.1
            )
        }
        Err(e) => println!("parse input failed. {}", e),
    }
    Ok(())
}

struct ReindeerPerformance {
    name: String,
    speed: u32,
    stamina: u32,
    full_cycle: u32,
}

struct PointsCounter {
    points: u32,
}

impl PointsCounter {
    fn new() -> PointsCounter {
        PointsCounter { points: 0 }
    }

    fn increase(&mut self) {
        self.points += 1
    }

    fn get(&self) -> u32 {
        self.points
    }
}
struct ReindeerRace {
    performances: Vec<ReindeerPerformance>,
}

impl ReindeerRace {
    fn race(&self, time: u32) -> (&String, u32) {
        self.race_with_tie(time)[0]
    }

    fn race_with_tie(&self, time: u32) -> Vec<(&String, u32)> {
        let mut r = self
            .performances
            .iter()
            .map(|p| {
                let full_distance = p.speed * p.stamina * (time / p.full_cycle);
                let remain = time % p.full_cycle;
                if remain > p.stamina {
                    return (&p.name, full_distance + p.speed * p.stamina);
                }
                return (&p.name, full_distance + p.speed * remain);
            })
            .collect::<Vec<(&String, u32)>>();
        r.sort_by(|a, b| a.1.cmp(&b.1).reverse());
        r.iter()
            .take_while(|p| p.1 == r[0].1)
            .map(|p| p.clone())
            .collect::<Vec<(&String, u32)>>()
    }

    fn race_mode_two(&self, time: u32) -> (&String, u32) {
        let mut points = self
            .performances
            .iter()
            .map(|p| (&p.name, PointsCounter::new()))
            .collect::<HashMap<&String, PointsCounter>>();
        for t in 1..time {
            for winner in self.race_with_tie(t) {
                points.get_mut(winner.0).unwrap().increase();
            }
        }
        let ret = points
            .iter()
            .max_by(|a, b| a.1.get().cmp(&b.1.get()))
            .unwrap();
        (ret.0, ret.1.get())
    }
}

impl FromStr for ReindeerPerformance {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return regex::Regex::new(
            r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.$",
        )
        .ok()
        .and_then(|re: regex::Regex| -> Option<ReindeerPerformance> {
            return re.captures(s).and_then(|c| {
                if c.len() < 4 {
                    return None;
                }
                let name = String::from(&c[1]);
                return c[2].parse::<u32>().ok().and_then(|speed| {
                    c[3].parse::<u32>().ok().and_then(|stamina| {
                        c[4].parse::<u32>().ok().and_then(|refill| {
                            return Some(ReindeerPerformance {
                                name,
                                speed,
                                stamina,
                                full_cycle: stamina + refill,
                            });
                        })
                    })
                });
            });
        })
        .ok_or(
            format!(
                "Reindeer performance input: \"{}\" is not in valid format",
                s
            )
            .into(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_race() {
        let performances = vec![
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        ]
        .iter()
        .map(|p| p.parse::<ReindeerPerformance>().unwrap())
        .collect::<Vec<ReindeerPerformance>>();
        let race = ReindeerRace { performances };
        let ret = race.race(1000);
        assert_eq!(1120, ret.1);
        assert_eq!("Comet", ret.0);
    }
    #[test]
    fn test_race_mode_2() {
        let performances = vec![
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        ]
        .iter()
        .map(|p| p.parse::<ReindeerPerformance>().unwrap())
        .collect::<Vec<ReindeerPerformance>>();
        let race = ReindeerRace { performances };
        let ret = race.race_mode_two(1000);
        assert_eq!(689, ret.1);
        assert_eq!("Dancer", ret.0);
    }
}
