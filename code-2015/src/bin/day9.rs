use std::{collections::HashMap, error, io, str::FromStr, vec};

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
        Err(e) => println!("parse instructions failed. {}", e),
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
                println!("xz11 {:?}", re.captures(s));
                return re.captures(s).and_then(|c| {
                    if c.len() < 4 {
                        println!("Asdfasdfsdf {}", c.len());
                        return None;
                    }
                    println!("asdfasd {} {} {}", &c[1], &c[2], &c[3]);
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
}

impl Map {
    fn new(routes: Vec<Route>) -> Map {
        let mut route_distances: HashMap<Location, HashMap<Location, u32>> = HashMap::new();

        for r in routes {
            if !route_distances.contains_key(&r.from) {
                route_distances.insert(r.from.clone(), HashMap::new());
            }

            route_distances
                .get_mut(&r.from)
                .unwrap()
                .insert(r.to.clone(), r.distance);

            if !route_distances.contains_key(&r.to) {
                route_distances.insert(r.to.clone(), HashMap::new());
            }

            route_distances
                .get_mut(&r.to)
                .unwrap()
                .insert(r.from.clone(), r.distance);
        }
        return Map { route_distances };
    }

    fn shortest_route(&self) -> u32 {
        return route_permutation(locations())
            .iter()
            .map(|routes| -> u32 {
                return routes
                    .iter()
                    .zip(routes.iter().skip(1))
                    .inspect(|(a, b)| println!("sdf {:?} {:?}", *a, *b))
                    .map(|(from, to)| self.route_distances.get(from).unwrap().get(to).unwrap())
                    .sum();
            })
            .min()
            .unwrap();
    }

    fn longest_route(&self) -> u32 {
        return route_permutation(locations())
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

fn locations() -> Vec<Location> {
    vec![
        Location::Tristram,
        Location::AlphaCentauri,
        Location::Snowdin,
        Location::Tambi,
        Location::Faerun,
        Location::Norrath,
        Location::Straylight,
        Location::Arbre,
    ]
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
    fn test_circuit() {
        let locations = vec![Location::Tambi, Location::Tristram, Location::AlphaCentauri];
        let permutations = route_permutation(locations);
        for p in permutations {
            println!("{:?}", &p);
        }
    }
    #[test]
    fn test_parse() {
        let re = regex::Regex::new(r"^(\w+) to (\w+) = (\d+)$").unwrap();
        let s = "Tristram to Arbre = 132";
        println!("xz11 {:?}", re.captures(s));

        // println!("asdf {:?}", regex::Regex::new(r"(.*) to (.*) = (\d)?$"));
        // println!(
        //     "sdf {:?}",
        //     "Tristram to AlphaCentauri = 34".parse::<Route>()
        // );
    }
}
