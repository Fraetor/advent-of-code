fn main() {
    let word_search = load("../input");
    println!("--- START ---");
    for line in &word_search {
        println!("{}", line.iter().collect::<String>());
    }
    println!("---  END  ---");

    let mut xmas_total = 0;
    xmas_total += find_horizontal(&word_search);
    println!("XMAS total: {xmas_total}");
    xmas_total += find_vertical(&word_search);
    println!("XMAS total: {xmas_total}");
    xmas_total += find_diagonally_upwards(&word_search);
    println!("XMAS total: {xmas_total}");
    xmas_total += find_diagonally_downwards(&word_search);
    println!("XMAS total: {xmas_total}");
}

/// 2D array of characters.
type CharGrid = Vec<Vec<char>>;

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

/// Count how many XMASs appear horizontally.
fn find_horizontal(c: &CharGrid) -> i32 {
    let mut xmas_count = 0;
    for row_chars in c {
        let row: String = row_chars.into_iter().collect();
        xmas_count += row.matches("XMAS").count();
        xmas_count += row.matches("SAMX").count();
    }
    return xmas_count.try_into().unwrap();
}

/// Count how many XMASs appear vertically.
fn find_vertical(c: &CharGrid) -> i32 {
    let mut xmas_count = 0;
    for column_index in 0..c[0].len() {
        let column: String = c.iter().map(|row| row[column_index]).collect();
        xmas_count += column.matches("XMAS").count();
        xmas_count += column.matches("SAMX").count();
    }
    return xmas_count.try_into().unwrap();
}

// Count how many XMASs appear diagonally upwards.
fn find_diagonally_upwards(c: &CharGrid) -> i32 {
    let rows = c.len();
    let columns = c[0].len();
    eprintln!("Rows: {rows}, Columns: {columns}");
    let mut xmas_count = 0;

    let mut start_points = Vec::new();
    for i in 0..rows {
        start_points.push(Point { i: i, j: 0 });
    }
    for j in 1..columns {
        start_points.push(Point { i: rows - 1, j: j });
    }

    // println!("{:?}", start_points);

    for start_point in start_points {
        let mut diag_chars = Vec::with_capacity(usize::max(rows, columns));
        for offset in 0..usize::max(rows, columns) {
            let i = match start_point.i.checked_sub(offset) {
                Some(x) => x,
                None => continue,
            };
            let j = start_point.j + offset;
            if i >= rows || j >= columns {
                continue;
            }
            diag_chars.push(c[i][j]);
            // eprintln!("({i}, {j}) => {}", diag_chars.last().unwrap());
        }
        let old_xmas_count = xmas_count;
        let diag: String = diag_chars.into_iter().collect();
        xmas_count += diag.matches("XMAS").count();
        xmas_count += diag.matches("SAMX").count();
        eprintln!("{start_point:?} {diag}: {}", xmas_count - old_xmas_count);
    }
    return xmas_count.try_into().unwrap();
}

// y = x or y = -x

#[derive(Debug)]
struct Point {
    i: usize,
    j: usize,
}

// Count how many XMASs appear diagonally downward.
fn find_diagonally_downwards(c: &CharGrid) -> i32 {
    let rows = c.len();
    let columns = c[0].len();
    eprintln!("Rows: {rows}, Columns: {columns}");
    let mut xmas_count = 0;

    let mut start_points = Vec::new();
    for i in 0..rows {
        start_points.push(Point { i: i, j: 0 });
    }
    for j in 1..columns {
        start_points.push(Point { i: 0, j: j });
    }

    // println!("{:?}", start_points);

    for start_point in start_points {
        let mut diag_chars = Vec::with_capacity(usize::max(rows, columns));
        for offset in 0..usize::max(rows, columns) {
            let i = start_point.i + offset;
            let j = start_point.j + offset;
            if i >= rows || j >= columns {
                continue;
            }
            diag_chars.push(c[i][j]);
            // eprintln!("({i}, {j}) => {}", diag_chars.last().unwrap());
        }
        let old_xmas_count = xmas_count;
        let diag: String = diag_chars.into_iter().collect();
        xmas_count += diag.matches("XMAS").count();
        xmas_count += diag.matches("SAMX").count();
        eprintln!("{start_point:?} {diag}: {}", xmas_count - old_xmas_count);
    }
    return xmas_count.try_into().unwrap();
}
