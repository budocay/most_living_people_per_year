use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
struct Detailed {
    name: String,
    dob: i32,
    dod: i32,
}

fn main() {
    let json = r#"
	[
		{
			"name": "Violet",
			"dob": 1950,
			"dod": 1972
		},
		{
			"name": "Tamqrah",
			"dob": 1930,
			"dod": 1981
		},
		{
			"name": "Tamqrah",
			"dob": 1972,
			"dod": 1981
		},
		{
			"name": "Tamqrah",
			"dob": 1972,
			"dod": 1980
		},
		{
			"name": "Viviene",
			"dob": 1999,
			"dod": 2023
		}
	]
  "#;

    let mut persons = parse_json(json);
    persons.sort_by(|a, b| b.dob.cmp(&a.dob));
    persons.reverse();
    let mut hashmap: HashMap<i32, usize> = HashMap::new();
    let mut prev_alive = 0;

    let mut current_year = persons.iter().map(|d| d.dob).min().unwrap();
    let max_year = persons.iter().map(|d| d.dod).max().unwrap();

    while current_year <= max_year {
        let alive = persons.iter().filter(|&n| n.dob == current_year).count();
        let dead = persons.iter().filter(|&n| n.dod == current_year).count();
        let currently_alive = prev_alive - dead + alive;
        hashmap.insert(current_year, currently_alive);
        prev_alive = currently_alive;
        current_year += 1;
    }
    let value = get_date_most_living_people(&hashmap);
    println!("date: {}", value);
}

fn get_date_most_living_people(hashmap: &HashMap<i32, usize>) -> i32
{
    let res = hashmap.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(k, _v)| k).unwrap();

    return *res
}

fn parse_json(json: &str) -> Vec<Detailed>
{
    let persons = serde_json::from_str::<Vec<Detailed>>(json).unwrap();
    return persons
}
