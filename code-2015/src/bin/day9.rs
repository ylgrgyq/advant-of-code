use std::{
    collections::{HashMap, HashSet},
    error, io,
    str::FromStr,
    vec,
};

fn main() -> io::Result<()> {
    match io::stdin().lines().collect::<Result<Vec<String>, _>>() {
        Ok(lines) => {
            let routes = lines
                .iter()
                .map(|s| s.parse::<Route>().unwrap())
                .collect::<Vec<Route>>();
            let m = Map::new(routes);
            println!("shortest: {}", m.shortest_route());
            println!("longest: {}", m.longest_route());
        }
        Err(e) => println!("parse route failed. {}", e),
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Location {
    Tristram,
    AlphaCentauri,
    Snowdin,
    Tambi,
    Faerun,
    Norrath,
    Straylight,
    Arbre,
}

impl FromStr for Location {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Tristram" => Ok(Location::Tristram),
            "AlphaCentauri" => Ok(Location::AlphaCentauri),
            "Snowdin" => Ok(Location::Snowdin),
            "Tambi" => Ok(Location::Tambi),
            "Faerun" => Ok(Location::Faerun),
            "Norrath" => Ok(Location::Norrath),
            "Straylight" => Ok(Location::Straylight),
            "Arbre" => Ok(Location::Arbre),
            _ => Err(format!("parse Location from: \"{}\" failed", s).into()),
        }
    }
}

#[derive(Clone, Debug)]
struct Route {
    from: Location,
    to: Location,
    distance: u32,
}

impl FromStr for Route {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return regex::Regex::new(r"^(\w+) to (\w+) = (\d+)$")
            .ok()
            .and_then(|re: regex::Regex| -> Option<Route> {
                return re.captures(s).and_then(|c| {
                    if c.len() < 4 {
                        return None;
                    }
                    return c[1].parse::<Location>().ok().and_then(|from| {
                        c[2].parse::<Location>().ok().and_then(|to| {
                            c[3].parse::<u32>()
                                .ok()
                                .and_then(|distance| Some(Route { from, to, distance }))
                        })
                    });
                });
            })
            .ok_or(format!("parse Route from: \"{}\" failed", s).into());
    }
}

struct Map {
    route_distances: HashMap<Location, HashMap<Location, u32>>,
    locations: Vec<Location>,
}

impl Map {
    fn new(routes: Vec<Route>) -> Map {
        let mut route_distances: HashMap<Location, HashMap<Location, u32>> = HashMap::new();

        let mut locations = HashSet::new();
        for r in routes {
            locations.insert(r.from.clone());
            locations.insert(r.to.clone());
            record_route_distance(&mut route_distances, &r.from, &r.to, r.distance);
            record_route_distance(&mut route_distances, &r.to, &r.from, r.distance);
        }
        return Map {
            route_distances,
            locations: locations.into_iter().collect(),
        };
    }

    fn shortest_route(&self) -> u32 {
        if self.locations.len() == 2 {
            return self
                .route_distances
                .get(&self.locations[0])
                .unwrap()
                .get(&self.locations[1])
                .unwrap()
                .clone();
        }
        return route_permutation(self.locations.clone())
            .iter()
            .map(|routes| -> u32 {
                return routes
                    .iter()
                    .zip(routes.iter().skip(1))
                    .map(|(from, to)| self.route_distances.get(from).unwrap().get(to).unwrap())
                    .sum();
            })
            .min()
            .unwrap();
    }

    fn longest_route(&self) -> u32 {
        return route_permutation(self.locations.clone())
            .iter()
            .map(|routes| -> u32 {
                return routes
                    .iter()
                    .zip(routes.iter().skip(1))
                    .map(|(from, to)| self.route_distances.get(from).unwrap().get(to).unwrap())
                    .sum();
            })
            .max()
            .unwrap();
    }
}

fn record_route_distance(
    route_distances: &mut HashMap<Location, HashMap<Location, u32>>,
    from: &Location,
    to: &Location,
    distance: u32,
) {
    if !route_distances.contains_key(&from) {
        route_distances.insert(from.clone(), HashMap::new());
    }

    route_distances
        .get_mut(&from)
        .unwrap()
        .insert(to.clone(), distance);
}

fn route_permutation(locations: Vec<Location>) -> Vec<Vec<Location>> {
    if locations.len() < 2 {
        panic!("not enough locations to compute combinations");
    }
    if locations.len() == 2 {
        return vec![
            vec![locations[0].clone(), locations[1].clone()],
            vec![locations[1].clone(), locations[0].clone()],
        ];
    }

    let mut ret = vec![];
    let route = &locations[0];
    let sub_permutations = route_permutation(locations[1..].to_vec());
    for p in sub_permutations {
        let mut i = 0;
        loop {
            if i > p.len() {
                break;
            }
            let mut p2 = p.clone();
            p2.insert(i, route.clone());
            ret.push(p2);
            i += 1;
        }
    }

    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_single_route_distance() {
        let routes = vec!["Tristram to Arbre = 132"]
            .iter()
            .map(|s| s.parse::<Route>().unwrap())
            .collect::<Vec<Route>>();
        let map = Map::new(routes);
        assert_eq!(map.shortest_route(), 132);
        assert_eq!(map.longest_route(), 132);
    }
    #[test]
    fn test_three_routes_distance() {
        let routes = vec![
            "Tristram to Arbre = 132",
            "Tristram to Faerun = 21",
            "Arbre to Faerun = 15",
        ]
        .iter()
        .map(|s| s.parse::<Route>().unwrap())
        .collect::<Vec<Route>>();
        let map = Map::new(routes);
        assert_eq!(map.shortest_route(), 36);
        assert_eq!(map.longest_route(), 153);
    }
}
