use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use rand::Rng;

mod framebuffer;
use framebuffer::Framebuffer;

#[derive(Clone, Copy)]
enum Organism {
    Block,
    Beehive,
    Loaf,
    Boat,
    Tub,
    Blinker,
    Toad,
    Beacon,
    Pulsar,
    Pentadecathlon,
    Glider,
    LWSS,
    MWSS,
    HWSS,
}

fn generate_random_pattern(grid: &mut Vec<Vec<bool>>, width: usize, height: usize) {
    let mut rng = rand::thread_rng();
    let organisms = [
        Organism::Block, Organism::Beehive, Organism::Loaf, Organism::Boat, Organism::Tub,
        Organism::Blinker, Organism::Toad, Organism::Beacon, Organism::Pulsar, Organism::Pentadecathlon,
        Organism::Glider, Organism::LWSS, Organism::MWSS, Organism::HWSS,
    ];

    let organism_count = rng.gen_range(50..100); 

    for _ in 0..organism_count {
        let organism = organisms[rng.gen_range(0..organisms.len())];
        let x = rng.gen_range(0..(width)); 
        let y = rng.gen_range(0..height);

        place_organism(grid, x, y, organism);
    }

    //agregar más Pulsars, Pentadecathlons y Blinkers (favs)
    let special_patterns = [
        Organism::Pulsar, Organism::Pentadecathlon, Organism::Blinker
    ];

    for _ in 0..5 { 
        let organism = special_patterns[rng.gen_range(0..special_patterns.len())];
        let x = rng.gen_range(0..(width));
        let y = rng.gen_range(0..height);

        if place_organism(grid, x, y, organism) {
        }
    }
}

fn place_organism(grid: &mut Vec<Vec<bool>>, x: usize, y: usize, organism: Organism) -> bool {
    match organism {
        Organism::Block => {
            if x + 1 < grid[0].len() && y + 1 < grid.len() {
                grid[y][x] = true;
                grid[y][x + 1] = true;
                grid[y + 1][x] = true;
                grid[y + 1][x + 1] = true;
                return true;
            }
        }
        Organism::Pulsar => {
            if x + 12 < grid[0].len() && y + 12 < grid.len() {
                let pulsar_pattern = [
                    (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
                    (0, 2), (5, 2), (7, 2), (12, 2),
                    (0, 3), (5, 3), (7, 3), (12, 3),
                    (0, 4), (5, 4), (7, 4), (12, 4),
                    (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
                    (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
                    (0, 8), (5, 8), (7, 8), (12, 8),
                    (0, 9), (5, 9), (7, 9), (12, 9),
                    (0, 10), (5, 10), (7, 10), (12, 10),
                    (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12)
                ];
                for &(dx, dy) in &pulsar_pattern {
                    grid[y + dy][x + dx] = true;
                }
                return true;
            }
        }
        Organism::Pentadecathlon => {
            if x + 14 < grid[0].len() && y + 4 < grid.len() {
                let pentadecathlon_pattern = [
                    (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6),
                    (1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (1, 6),
                    (2, 1), (2, 2), (2, 3), (2, 4), (2, 5), (2, 6),
                    (3, 1), (3, 2), (3, 3), (3, 4), (3, 5), (3, 6)
                ];
                for &(dx, dy) in &pentadecathlon_pattern {
                    grid[y + dy][x + dx] = true;
                }
                return true;
            }
        }
        Organism::Blinker => {
            if x + 2 < grid[0].len() && y + 2 < grid.len() {
                let blinker_pattern = [
                    (0, 1), (1, 1), (2, 1)
                ];
                for &(dx, dy) in &blinker_pattern {
                    grid[y + dy][x + dx] = true;
                }
                return true;
            }
        }
        Organism::Tub => {
            if x + 2 < grid[0].len() && y + 2 < grid.len() {
                let tub_pattern = [
                    (1, 0),
                    (0, 1), (2, 1),
                    (1, 2)
                ];
                for &(dx, dy) in &tub_pattern {
                    grid[y + dy][x + dx] = true;
                }
                return true;
            }
        }
        _ => {}
    }
    false
}

fn render(framebuffer: &mut Framebuffer, grid: &Vec<Vec<bool>>) {
    let background_color = 0x00330099;
    let cell_color = 0x00FFCC00; 

    framebuffer.set_background_color(background_color); 
    framebuffer.clear(); 

    framebuffer.set_current_color(cell_color); 
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            if grid[y][x] {
                framebuffer.point(x as isize, y as isize);
            }
        }
    }
}

fn count_neighbors(grid: &Vec<Vec<bool>>, x: usize, y: usize, width: usize, height: usize) -> usize {
    let mut count = 0;
    for i in [-1, 0, 1].iter() {
        for j in [-1, 0, 1].iter() {
            if *i == 0 && *j == 0 {
                continue;
            }
            let nx = (x as isize + i) as usize;
            let ny = (y as isize + j) as usize;
            if nx < width && ny < height && grid[ny][nx] {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 100;
    let framebuffer_height = 100;

    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let mut grid = vec![vec![false; framebuffer_width]; framebuffer_height];

    // Generar patrón inicial aleatorio con más patrones especiales
    generate_random_pattern(&mut grid, framebuffer_width, framebuffer_height);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Actualizar framebuffer
        render(&mut framebuffer, &grid);

        // Actualizar la ventana con el contenido del framebuffer
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        // Calcular el siguiente estado
        let mut new_grid = grid.clone();
        for y in 0..framebuffer_height {
            for x in 0..framebuffer_width {
                let neighbors = count_neighbors(&grid, x, y, framebuffer_width, framebuffer_height);
                if grid[y][x] {
                    if neighbors < 2 || neighbors > 3 {
                        new_grid[y][x] = false;
                    }
                } else {
                    if neighbors == 3 {
                        new_grid[y][x] = true;
                    }
                }
            }
        }
        grid = new_grid;

        std::thread::sleep(frame_delay);
    }
}

