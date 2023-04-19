use clap::Parser;
use macroquad::prelude::*;

#[derive(Parser, Debug)]
struct Args {
    // The file to load
    #[clap(short, long)]
    file: String,

    #[clap(short, long, require_equals = true, possible_values = &["1", "2", "3", "4", "5"])]
    speed: String,
}

type LifeGrid = Vec<Vec<i32>>;

#[macroquad::main("BasicShapes")]
async fn main() {
    let args = Args::parse();
    let filepath = &args.file;

    let slowdown_factor = match args.speed.as_str() {
        "1" => 7,
        "2" => 5,
        "3" => 3,
        "4" => 2,
        "5" => 1,

        // Should be impossible due to clap's handling of possible values
        _ => panic!("Error: speed_factor must be between 1 and 5, inclusive"),
    };

    let mut map1: LifeGrid = parse_file(&filepath);
    let mut map2: LifeGrid = map1.clone();

    let width = map1[0].len() as f32 * 10.0;
    let height = map1.len() as f32 * 10.0;
    let cols = map1[0].len() as i32;
    let rows = map1.len() as i32;

    println!("Grid dimensions: H: {}, W: {}", rows, cols);

    let mut i: u64 = 0;
    loop {
        macroquad::window::request_new_screen_size(width, height);
        clear_background(BLACK);

        if i % slowdown_factor == 0 {
            advance_map(&mut map1, &mut map2, rows, cols);
        }

        for (r, row) in map2.iter().enumerate() {
            for (c, val) in row.iter().enumerate() {
                if *val == 1 {
                    draw_rectangle(c as f32 * 10.0, r as f32 * 10.0, 10.0, 10.0, WHITE);
                }
            }
        }

        draw_text(&i.to_string(), 0.0, height, 20.0, WHITE);

        i += 1;
        next_frame().await
    }
}

fn parse_file(filename: &str) -> Vec<Vec<i32>> {
    let contents = std::fs::read_to_string(filename);
    let s = match contents {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    let mut map: LifeGrid = vec![];

    for (i, line) in s.lines().enumerate() {
        if line.is_empty() {
            break;
        }
        map.push(vec![]);
        for c in line.chars() {
            if c == '#' {
                map[i].push(1);
            } else if c == '_' {
                map[i].push(0);
            }
        }
    }

    map
}

fn advance_map(curr_map: &mut LifeGrid, next_map: &mut LifeGrid, rows: i32, cols: i32) {
    let dir_rows: Vec<i32> = vec![-1, -1, -1, 0, 0, 1, 1, 1];
    let dir_cols: Vec<i32> = vec![-1, 0, 1, -1, 1, -1, 0, 1];

    for r in 0..rows {
        for c in 0..cols {
            let mut num_living_neighbors = 0;
            for d in 0..8 {
                let rr: i32 = r + dir_rows[d];
                let cc: i32 = c + dir_cols[d];
                if 0 <= rr
                    && rr < rows
                    && 0 <= cc
                    && cc < cols
                    && curr_map[rr as usize][cc as usize] == 1
                {
                    num_living_neighbors += 1;
                }
            }
            if curr_map[r as usize][c as usize] == 1
                && (num_living_neighbors == 2 || num_living_neighbors == 3)
            {
                // Do nothing
            } else if curr_map[r as usize][c as usize] == 0 && num_living_neighbors == 3 {
                next_map[r as usize][c as usize] = 1;
            } else {
                next_map[r as usize][c as usize] = 0;
            }
        }
    }
    // Advance
    *curr_map = next_map.clone();
}
