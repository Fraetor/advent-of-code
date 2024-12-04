/// 2D array of characters.
type CharGrid = Vec<Vec<char>>;

#[derive(Debug)]
struct Point {
    i: usize,
    j: usize,
}

fn main() {
    let word_search = load("../input");
    print_grid(&word_search);

    let xmas_total = find_windowed(&word_search);
    println!("X-MAS total: {xmas_total}");
}

/// Load word search from a file.
fn load(filename: &str) -> CharGrid {
    let raw_input = std::fs::read_to_string(filename).unwrap();
    let letter_grid = raw_input
        .lines()
        .filter_map(|s| {
            if s.trim().len() > 1 {
                Some(s.chars().collect())
            } else {
                None
            }
        })
        .collect();
    return letter_grid;
}

fn print_grid(c: &CharGrid) {
    for row in c {
        println!("{}", row.iter().collect::<String>());
    }
}

/// Search 3x3 windows, returning the number of matches.
fn find_windowed(c: &CharGrid) -> i32 {
    let rows = c.len();
    let columns = c[0].len();
    eprintln!("Rows: {rows}, Columns: {columns}");

    let mut start_points = Vec::new();
    for i in 0..rows - 2 {
        for j in 0..columns - 2 {
            start_points.push(Point { i: i, j: j });
        }
    }

    let mut xmas_count = 0;
    for sp in start_points {
        eprintln!("{sp:?}");
        let window = vec![
            vec![c[sp.i][sp.j], c[sp.i][sp.j + 1], c[sp.i][sp.j + 2]],
            vec![
                c[sp.i + 1][sp.j],
                c[sp.i + 1][sp.j + 1],
                c[sp.i + 1][sp.j + 2],
            ],
            vec![
                c[sp.i + 2][sp.j],
                c[sp.i + 2][sp.j + 1],
                c[sp.i + 2][sp.j + 2],
            ],
        ];
        if check_window(&window) {
            xmas_count += 1;
        }
    }

    return xmas_count;
}

/// Checks if a 3x3 grid contains an X-MAS.
fn check_window(w: &CharGrid) -> bool {
    let mas_1: String = vec![w[0][0], w[1][1], w[2][2]].into_iter().collect();
    let mas_2: String = vec![w[0][2], w[1][1], w[2][0]].into_iter().collect();
    let m1 = mas_1 == "MAS" || mas_1 == "SAM";
    let m2 = mas_2 == "MAS" || mas_2 == "SAM";
    let is_mas = m1 && m2;
    print_grid(w);
    eprintln!("{mas_1}: {m1} | {mas_2}: {m2}\n=======================");

    return is_mas;
}
