use std::collections::HashSet;

static mut CONTAINS_COUNTER: usize = 0;
static mut TILE_AT_COUNTER: usize = 0;
static mut STEP_COUNTER: usize = 0;

fn main() {
    let input_file = "input";

    let p1_total = part_1(input_file);
    println!("Part 1 total: {p1_total}");

    let p2_total = part_2(input_file);
    println!("Part 2 total: {p2_total}");

    unsafe {
        eprintln!(
            "contains called {CONTAINS_COUNTER} times.\ntile_at called {TILE_AT_COUNTER} times.\nstep called {STEP_COUNTER} times."
        );
    }
}

/// Number of distinct positions visited by the guard.
fn part_1(filename: &str) -> usize {
    let mut visited_positions = HashSet::new();
    let (map, mut guard) = load(filename);

    while map.contains(&guard.position) {
        visited_positions.insert(guard.position);
        // eprintln!("{}", &guard.display_on_map(&map));
        guard.step(&map);
        // eprintln!("{:?}", &guard);
        // std::thread::sleep(std::time::Duration::from_millis(500));
    }

    return visited_positions.len();
}

/// Number of distinct positions where a new obstacle could be added to cause
/// the guard to loop.
fn part_2(filename: &str) -> usize {
    let (map, guard) = load(filename);
    // As for part one calculate list of visited positions. We only need to try
    // these for obstacle placement.
    let mut visited_positions =
        HashSet::with_capacity((map.height * map.width).try_into().unwrap());
    let mut temp_guard = guard;
    while map.contains(&temp_guard.position) {
        visited_positions.insert(temp_guard.position);
        temp_guard.step(&map);
    }
    visited_positions.shrink_to_fit();

    let looping_obstacle_position_count = visited_positions
        .into_iter()
        .filter(|p| *p != guard.position)
        .filter(|p| map.tile_at(p) == Tile::Free)
        .filter(|p| check_looping_obstacle(&guard, &map, p))
        .count();
    return looping_obstacle_position_count;
}

fn check_looping_obstacle(guard: &Guard, map: &Map, obstacle_position: &Position) -> bool {
    let mut modified_map = map.clone();
    modified_map.add_obstacle_at(obstacle_position);
    // eprintln!("{}", guard.display_on_map(&modified_map));
    return is_looping(&mut guard.clone(), &modified_map);
}

fn is_looping(guard: &mut Guard, map: &Map) -> bool {
    let mut guard_history = HashSet::with_capacity((map.height * map.width).try_into().unwrap());
    while map.contains(&guard.position) {
        if !guard_history.insert(*guard) {
            return true;
        }
        guard.step(&map);
    }
    return false;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Facing {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Free,
    Obstruction,
    OutOfBounds,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

/// Position of a guard.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Guard {
    position: Position,
    direction: Facing,
}

impl Guard {
    /// Advance the guard in the map.
    ///
    /// Guard movement rules:
    /// If there is something directly in front of you, turn right 90 degrees.
    /// Otherwise, take a step forward.
    fn step(&mut self, map: &Map) {
        unsafe {
            STEP_COUNTER += 1;
        }
        let mut tries = 0;
        self.position = loop {
            let new_position = match self.direction {
                Facing::North => Position {
                    y: self.position.y - 1,
                    ..self.position
                },
                Facing::East => Position {
                    x: self.position.x + 1,
                    ..self.position
                },
                Facing::South => Position {
                    y: self.position.y + 1,
                    ..self.position
                },
                Facing::West => Position {
                    x: self.position.x - 1,
                    ..self.position
                },
            };
            let current_tile = map.tile_at(&new_position);
            if current_tile == Tile::Free || current_tile == Tile::OutOfBounds {
                break Some(new_position);
            } else {
                // eprintln!("No path in {:?} direction.", self.direction);
                // Rotate clockwise if obstacle encountered.
                self.direction = match self.direction {
                    Facing::North => Facing::East,
                    Facing::East => Facing::South,
                    Facing::South => Facing::West,
                    Facing::West => Facing::North,
                };
            }
            tries += 1;
            // We have tried all directions and found nowhere to go.
            if tries > 3 {
                break None;
            }
        }
        .expect(&format!(
            "Cannot move from position {:?}! Guard should not be enclosed.",
            self.position
        ));
    }

    fn display_on_map(&self, map: &Map) -> String {
        let mut map_string = String::new();
        for y in 0..map.height {
            for x in 0..map.width {
                if self.position == (Position { x, y }) {
                    map_string.push(match self.direction {
                        Facing::North => '^',
                        Facing::East => '>',
                        Facing::South => 'v',
                        Facing::West => '<',
                    });
                } else {
                    map_string.push(
                        match map.tiles[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()] {
                            Tile::Free => '.',
                            Tile::Obstruction => '#',
                            Tile::OutOfBounds => {
                                panic!("Should not be printing out of bound tiles.")
                            }
                        },
                    );
                }
            }
            map_string.push('\n');
        }

        return map_string;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Map {
    width: i32,
    height: i32,
    // (0,0) is Northwest corner.
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn contains(&self, position: &Position) -> bool {
        unsafe {
            CONTAINS_COUNTER += 1;
        }
        0 <= position.x && position.x < self.width && 0 <= position.y && position.y < self.height
    }

    fn tile_at(&self, position: &Position) -> Tile {
        unsafe {
            TILE_AT_COUNTER += 1;
        }
        if !self.contains(position) {
            return Tile::OutOfBounds;
        }
        self.tiles[usize::try_from(position.y).unwrap()][usize::try_from(position.x).unwrap()]
    }

    fn add_obstacle_at(&mut self, position: &Position) {
        self.tiles[usize::try_from(position.y).unwrap()][usize::try_from(position.x).unwrap()] =
            Tile::Obstruction;
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map_string = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                map_string.push(
                    match self.tiles[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()] {
                        Tile::Free => '.',
                        Tile::Obstruction => '#',
                        Tile::OutOfBounds => panic!("Should not be printing out of bound tiles."),
                    },
                );
            }
            map_string.push('\n');
        }
        write!(f, "{}", map_string)
    }
}

/// Load the map and guard's starting position.
fn load(filename: &str) -> (Map, Guard) {
    let raw_input = std::fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = raw_input.lines().filter(|l| !l.is_empty()).collect();
    let mut rows = Vec::new();
    let mut guard_position = None;
    for line in lines {
        let mut row = Vec::new();
        for character in line.chars() {
            row.push(match character {
                '.' => Tile::Free,
                '#' => Tile::Obstruction,
                '^' => {
                    guard_position = Some(Position {
                        x: row.len().try_into().unwrap(),
                        y: rows.len().try_into().unwrap(),
                    });
                    Tile::Free
                }
                _ => panic!(
                    "Character should be one of '.', '#', '^' not '{}'",
                    character
                ),
            });
        }
        rows.push(row);
    }
    let map = Map {
        width: rows[0].len().try_into().unwrap(),
        height: rows.len().try_into().unwrap(),
        tiles: rows,
    };

    let guard = Guard {
        position: guard_position.expect("There should be a guard on the map."),
        direction: Facing::North,
    };
    return (map, guard);
}
