use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use anyhow::{anyhow, Result};
use enum_map::{enum_map, Enum};
use itertools::Itertools;

#[derive(Debug)]
struct Hall {
    y_pos: usize,
    x_positions: Range<usize>,
}

impl Hall {
    fn contains(&self, pos: (usize, usize)) -> bool {
        self.x_positions.contains(&pos.0) && self.y_pos == pos.1
    }
}

#[derive(Debug)]
struct Room {
    x_pos: usize,
    y_positions: Range<usize>,
    amphipod_type: AmphipodType,
}

impl Room {
    fn contains(&self, pos: (usize, usize)) -> bool {
        self.x_pos == pos.0 && self.y_positions.contains(&pos.1)
    }
}

#[derive(Debug)]
struct Map {
    hall: Hall,
    rooms: enum_map::EnumMap<AmphipodType, Room>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Enum, Hash)]
enum AmphipodType {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl AmphipodType {
    #[allow(dead_code)]
    fn from_char(c: char) -> Result<Self> {
        match c {
            'A' => Ok(AmphipodType::Amber),
            'B' => Ok(AmphipodType::Bronze),
            'C' => Ok(AmphipodType::Copper),
            'D' => Ok(AmphipodType::Desert),
            _ => Err(anyhow!("Invalid amphipod type {}", c)),
        }
    }

    fn to_char(self) -> char {
        match self {
            AmphipodType::Amber => 'A',
            AmphipodType::Bronze => 'B',
            AmphipodType::Copper => 'C',
            AmphipodType::Desert => 'D',
        }
    }

    fn movement_cost(&self) -> usize {
        match self {
            AmphipodType::Amber => 1,
            AmphipodType::Bronze => 10,
            AmphipodType::Copper => 100,
            AmphipodType::Desert => 1000,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Amphipod {
    type_: AmphipodType,
    pos: (usize, usize),
}

fn parse_input(_input: &str) -> (Map, Vec<Amphipod>) {
    let hall = Hall {
        y_pos: 1,
        x_positions: 1..12,
    };

    let amphipods = vec![
        Amphipod {
            type_: AmphipodType::Bronze,
            pos: (3, 2),
        },
        Amphipod {
            type_: AmphipodType::Copper,
            pos: (3, 3),
        },
        Amphipod {
            type_: AmphipodType::Copper,
            pos: (5, 2),
        },
        Amphipod {
            type_: AmphipodType::Desert,
            pos: (5, 3),
        },
        Amphipod {
            type_: AmphipodType::Amber,
            pos: (7, 2),
        },
        Amphipod {
            type_: AmphipodType::Desert,
            pos: (7, 3),
        },
        Amphipod {
            type_: AmphipodType::Bronze,
            pos: (9, 2),
        },
        Amphipod {
            type_: AmphipodType::Amber,
            pos: (9, 3),
        },
    ];

    (
        Map {
            hall,
            rooms: enum_map! {
                AmphipodType::Amber => Room {
                    x_pos: 3,
                    y_positions: 2..4,
                    amphipod_type: AmphipodType::Amber,
                },
                AmphipodType::Bronze => Room {
                    x_pos: 5,
                    y_positions: 2..4,
                    amphipod_type: AmphipodType::Bronze,
                },
                AmphipodType::Copper => Room {
                    x_pos: 7,
                    y_positions: 2..4,
                    amphipod_type: AmphipodType::Copper,
                },
                AmphipodType::Desert => Room {
                    x_pos: 9,
                    y_positions: 2..4,
                    amphipod_type: AmphipodType::Desert,
                },
            },
        },
        amphipods,
    )
}

fn parse_input_2(_input: &str) -> (Map, Vec<Amphipod>) {
    let hall = Hall {
        y_pos: 1,
        x_positions: 1..12,
    };

    let amphipods = vec![
        Amphipod {
            type_: AmphipodType::Bronze,
            pos: (3, 2),
        },
        Amphipod {
            type_: AmphipodType::Desert,
            pos: (3, 3),
        },
        Amphipod {
            type_: AmphipodType::Desert,
            pos: (3, 4),
        },
        Amphipod {
            type_: AmphipodType::Copper,
            pos: (3, 5),
        },
        Amphipod {
            type_: AmphipodType::Copper,
            pos: (5, 2),
        },
        Amphipod {
            type_: AmphipodType::Copper,
            pos: (5, 3),
        },
        Amphipod {
            type_: AmphipodType::Bronze,
            pos: (5, 4),
        },
        Amphipod {
            type_: AmphipodType::Desert,
            pos: (5, 5),
        },
        Amphipod {
            type_: AmphipodType::Amber,
            pos: (7, 2),
        },
        Amphipod {
            type_: AmphipodType::Bronze,
            pos: (7, 3),
        },
        Amphipod {
            type_: AmphipodType::Amber,
            pos: (7, 4),
        },
        Amphipod {
            type_: AmphipodType::Desert,
            pos: (7, 5),
        },
        Amphipod {
            type_: AmphipodType::Bronze,
            pos: (9, 2),
        },
        Amphipod {
            type_: AmphipodType::Amber,
            pos: (9, 3),
        },
        Amphipod {
            type_: AmphipodType::Copper,
            pos: (9, 4),
        },
        Amphipod {
            type_: AmphipodType::Amber,
            pos: (9, 5),
        },
    ];

    (
        Map {
            hall,
            rooms: enum_map! {
                AmphipodType::Amber => Room {
                    x_pos: 3,
                    y_positions: 2..6,
                    amphipod_type: AmphipodType::Amber,
                },
                AmphipodType::Bronze => Room {
                    x_pos: 5,
                    y_positions: 2..6,
                    amphipod_type: AmphipodType::Bronze,
                },
                AmphipodType::Copper => Room {
                    x_pos: 7,
                    y_positions: 2..6,
                    amphipod_type: AmphipodType::Copper,
                },
                AmphipodType::Desert => Room {
                    x_pos: 9,
                    y_positions: 2..6,
                    amphipod_type: AmphipodType::Desert,
                },
            },
        },
        amphipods,
    )
}

fn is_finished(amphipods: &[Amphipod], map: &Map) -> bool {
    amphipods
        .iter()
        .all(|amphipod| map.rooms[amphipod.type_].contains(amphipod.pos))
}

fn manhattan_distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
    p1.0.max(p2.0) - p1.0.min(p2.0) + p1.1.max(p2.1) - p1.1.min(p2.1)
}

#[allow(dead_code)]
fn print_state(amphipods: &[Amphipod], map: &Map) {
    let hall = map
        .hall
        .x_positions
        .clone()
        .map(|x| {
            if let Some(a) = amphipods.iter().find(|a| a.pos == (x, map.hall.y_pos)) {
                a.type_.to_char()
            } else {
                '.'
            }
        })
        .collect::<String>();
    println!("#{}#", hall);

    for y in map.rooms[AmphipodType::Amber].y_positions.clone() {
        let row = map
            .hall
            .x_positions
            .clone()
            .map(|x| {
                if let Some(a) = amphipods.iter().find(|a| a.pos == (x, y)) {
                    a.type_.to_char()
                } else if x == 3 || x == 5 || x == 7 || x == 9 {
                    '.'
                } else {
                    '#'
                }
            })
            .collect::<String>();
        println!("#{}#", row);
    }
    println!("\n");
}

fn calculate_moves(amphipods: &[Amphipod], map: &Map) -> Vec<(Vec<Amphipod>, usize)> {
    let mut new_states = Vec::new();

    // Loop through amphipods that aren't in their destination room
    for (i, amphipod) in amphipods.iter().enumerate() {
        // If amphipod is in a original room, they can move to the hallway
        if let Some(in_room) = map.rooms.values().find(|room| room.contains(amphipod.pos)) {
            // as long as they're not already in their dest room, or if they are but may need to move to let
            // another type of amphipod leave
            let other_types_in_room = in_room
                .y_positions
                .clone()
                .map(|y| (in_room.x_pos, y))
                .cartesian_product(amphipods)
                .any(|(pos, other)| pos == other.pos && other.type_ != in_room.amphipod_type);
            let can_reach_hallway = (map.hall.y_pos..amphipod.pos.1)
                .cartesian_product(amphipods)
                .all(|(y, other)| other.pos != (amphipod.pos.0, y));
            if can_reach_hallway && other_types_in_room {
                let mut new_pos = (amphipod.pos.0, map.hall.y_pos);
                while map.hall.contains(new_pos)
                    && !amphipods.iter().any(|other| other.pos == new_pos)
                {
                    if map.rooms.values().all(|room| room.x_pos != new_pos.0) {
                        let mut new_state = amphipods.to_vec();
                        new_state[i].pos = new_pos;
                        new_states.push((
                            new_state,
                            manhattan_distance(amphipod.pos, new_pos)
                                * amphipod.type_.movement_cost(),
                        ));
                    }
                    new_pos.0 -= 1;
                }

                new_pos = (amphipod.pos.0, map.hall.y_pos);
                while map.hall.contains(new_pos)
                    && !amphipods.iter().any(|other| other.pos == new_pos)
                {
                    if map.rooms.values().all(|room| room.x_pos != new_pos.0) {
                        let mut new_state = amphipods.to_vec();
                        new_state[i].pos = new_pos;
                        new_states.push((
                            new_state,
                            manhattan_distance(amphipod.pos, new_pos)
                                * amphipod.type_.movement_cost(),
                        ));
                    }
                    new_pos.0 += 1;
                }
            }
        } else if map.hall.contains(amphipod.pos) {
            // Ensure target room is empty or only contains same type amphipod
            let dest_room = &map.rooms[amphipod.type_];
            if amphipods
                .iter()
                .filter(|other| other.type_ != amphipod.type_)
                .all(|other| !dest_room.contains(other.pos))
            {
                // Ensure we can get from current pos to room entrance
                let can_reach_room = if dest_room.x_pos > amphipod.pos.0 {
                    ((amphipod.pos.0 + 1)..=dest_room.x_pos)
                        .cartesian_product(amphipods)
                        .all(|(x, other)| other.pos != (x, map.hall.y_pos))
                } else {
                    (dest_room.x_pos..amphipod.pos.0)
                        .cartesian_product(amphipods)
                        .all(|(x, other)| other.pos != (x, map.hall.y_pos))
                };
                if can_reach_room {
                    let dest_pos = dest_room
                        .y_positions
                        .clone()
                        .map(|y| (dest_room.x_pos, y))
                        .rev()
                        .find(|pos| amphipods.iter().all(|other| other.pos != *pos))
                        .expect("Room should have an empty tile");
                    let mut new_state = amphipods.to_vec();
                    new_state[i].pos = dest_pos;
                    new_states.push((
                        new_state,
                        manhattan_distance(amphipod.pos, dest_pos) * amphipod.type_.movement_cost(),
                    ));
                }
            }
        }
    }

    new_states
}

fn a_star(start: Vec<Amphipod>, map: &Map) -> Option<usize> {
    #[derive(Debug, Clone, Copy)]
    struct Cost {
        g: usize,
        h: usize,
    }

    /// Heuristic for remaining cost from position to end.
    /// This must be "admissable", meaning it cannot overestimate the cost.
    ///
    fn h(_amphipods: &[Amphipod]) -> usize {
        //pos.0.max(end.0) - pos.0.min(end.0) + pos.1.max(end.1) - pos.1.min(end.1)
        0
    }

    let mut open: HashMap<Vec<Amphipod>, Cost> = HashMap::default();
    let mut closed: HashSet<Vec<Amphipod>> = HashSet::default();
    let initial_h = h(&start);
    open.insert(start, Cost { g: 0, h: initial_h });

    while let Some((current_state, current_cost)) = open
        .iter()
        .min_by_key(|(_state, cost)| cost.g + cost.h)
        .map(|(state, cost)| (state.clone(), *cost))
    {
        open.remove(&current_state);
        closed.insert(current_state.clone());

        if is_finished(&current_state, map) {
            assert_eq!(current_cost.h, 0);
            return Some(current_cost.g);
        }

        // Calculate the cost for each neighbouring cell and add to open list.
        for (next_move, cost) in calculate_moves(&current_state, map)
            .iter()
            .filter(|(next_move, _cost)| !closed.contains(next_move))
        {
            let g = current_cost.g + cost;
            let h = h(next_move);
            open.entry(next_move.clone())
                .and_modify(|existing| {
                    assert_eq!(h, existing.h);
                    // If we've found a shorter route to an already discovered state, update its cost.
                    existing.g = g.min(existing.g);
                })
                .or_insert(Cost { g, h });
        }
    }

    None
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/day23")?;
    let (map, amphipods) = parse_input(&input);

    let result_a = a_star(amphipods, &map).ok_or_else(|| anyhow!("Failed to find path"))?;

    assert_eq!(result_a, 14350);
    println!("Day 23, part A: {}", result_a);

    let (map_2, amphipods_2) = parse_input_2(&input);

    let result_b = a_star(amphipods_2, &map_2).ok_or_else(|| anyhow!("Failed to find path"))?;
    assert_eq!(result_b, 49742);
    println!("Day 23, part B: {}", result_b);

    Ok(())
}
