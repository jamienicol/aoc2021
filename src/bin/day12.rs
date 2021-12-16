use std::collections::HashMap;

use anyhow::{anyhow, Result};

fn parse_connections(input: &str) -> Result<Vec<(String, String)>> {
    input
        .lines()
        .map(|line| {
            line.split_once('-')
                .ok_or_else(|| anyhow!("Invalid input"))
                .map(|(from, to)| (from.to_string(), to.to_string()))
        })
        .collect()
}

#[derive(Debug, Clone)]
struct SearchState {
    route: Vec<String>,
    small_visited_counts: HashMap<String, usize>,
    max_small_visited: usize,
}

impl SearchState {
    fn new() -> Self {
        let mut small_visited_counts = HashMap::default();
        small_visited_counts.insert("start".to_string(), 2);
        Self {
            route: vec!["start".to_string()],
            small_visited_counts,
            max_small_visited: 0,
        }
    }
}

fn find_routes(
    map: &HashMap<String, Vec<String>>,
    state: SearchState,
    allow_revisit_small: bool,
) -> Vec<Vec<String>> {
    let mut completed_routes = Vec::new();

    for option in map[state.route.last().unwrap()].iter().filter(|next| {
        next.chars().all(|c| c.is_uppercase())
            || (*state.small_visited_counts.get(*next).unwrap_or(&0) == 0
                || *state.small_visited_counts.get(*next).unwrap_or(&0) == 1
                    && state.max_small_visited == 1
                    && allow_revisit_small)
    }) {
        let mut state = state.clone();
        state.route.push(option.clone());

        if !option.chars().all(|c| c.is_uppercase()) && option != "start" {
            let visited_count = *state.small_visited_counts.get(option).unwrap_or(&0) + 1;
            state.max_small_visited = state.max_small_visited.max(visited_count);
            state
                .small_visited_counts
                .insert(option.clone(), visited_count);
        }

        if option == "end" {
            completed_routes.push(state.route);
        } else {
            completed_routes.extend(find_routes(map, state, allow_revisit_small));
        }
    }

    completed_routes
}

fn main() -> Result<()> {
    let input = &std::fs::read_to_string("res/day12")?;

    let mut map: HashMap<String, Vec<String>> = HashMap::default();
    for (from, to) in parse_connections(input)? {
        map.entry(from.clone()).or_default().push(to.clone());
        map.entry(to).or_default().push(from);
    }

    let result_a = find_routes(&map, SearchState::new(), false).len();
    assert_eq!(result_a, 4495);
    println!("Day 12, part A: {}", result_a);

    let result_b = find_routes(&map, SearchState::new(), true).len();
    assert_eq!(result_b, 131254);
    println!("Day 12, part B: {}", result_b);

    Ok(())
}
