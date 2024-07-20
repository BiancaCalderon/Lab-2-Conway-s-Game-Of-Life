use minifb::{Key, Window, WindowOptions};
use std::time::Duration; // AsegÃºrate de importar esto
use rand::Rng;
use crate::framebuffer::Framebuffer;
use crate::color::Color;

mod color;
mod framebuffer;

#[derive(Clone, Copy)]
enum Organism {
    Glider,
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
    LWSS,
    MWSS,
    HWSS,
}

impl Organism {
    const COUNT: usize = 14;

    fn from_index(index: usize) -> Self {
        match index {
            0 => Organism::Glider,
            1 => Organism::Block,
            2 => Organism::Beehive,
            3 => Organism::Loaf,
            4 => Organism::Boat,
            5 => Organism::Tub,
            6 => Organism::Blinker,
            7 => Organism::Toad,
            8 => Organism::Beacon,
            9 => Organism::Pulsar,
            10 => Organism::Pentadecathlon,
            11 => Organism::LWSS,
            12 => Organism::MWSS,
            13 => Organism::HWSS,
            _ => Organism::Glider,
        }
    }
}

fn initialize_pattern(framebuffer: &mut Framebuffer, count: usize) {
    let mut rng = rand::thread_rng();
    let max_x = framebuffer.width - 10;
    let max_y = framebuffer.height - 10;

    for _ in 0..count {
        let x = rng.gen_range(0..max_x);
        let y = rng.gen_range(0..max_y);
        let organism = Organism::from_index(rng.gen_range(0..Organism::COUNT));
        let color = match organism {
            Organism::Glider => Color::new(255, 255, 0),
            Organism::Block => Color::new(0, 255, 0),
            Organism::Beehive => Color::new(0, 0, 255),
            Organism::Loaf => Color::new(255, 0, 255),
            Organism::Boat => Color::new(0, 255, 255),
            Organism::Tub => Color::new(255, 128, 0),
            Organism::Blinker => Color::new(128, 0, 255),
            Organism::Toad => Color::new(0, 128, 255),
            Organism::Beacon => Color::new(128, 255, 128),
            Organism::Pulsar => Color::new(255, 0, 128),
            Organism::Pentadecathlon => Color::new(128, 128, 255),
            Organism::LWSS => Color::new(128, 255, 255),
            Organism::MWSS => Color::new(255, 128, 128),
            Organism::HWSS => Color::new(128, 255, 0),
        };
        add_pattern(framebuffer, x, y, organism, color);
    }
}

fn add_pattern(framebuffer: &mut Framebuffer, x: usize, y: usize, organism: Organism, color: Color) {
    framebuffer.set_current_color(color.to_u32());

    match organism {
        Organism::Glider => add_glider(framebuffer, x, y),
        Organism::Block => add_block(framebuffer, x, y),
        Organism::Beehive => add_beehive(framebuffer, x, y),
        Organism::Loaf => add_loaf(framebuffer, x, y),
        Organism::Boat => add_boat(framebuffer, x, y),
        Organism::Tub => add_tub(framebuffer, x, y),
        Organism::Blinker => add_blinker(framebuffer, x, y),
        Organism::Toad => add_toad(framebuffer, x, y),
        Organism::Beacon => add_beacon(framebuffer, x, y),
        Organism::Pulsar => add_pulsar(framebuffer, x, y),
        Organism::Pentadecathlon => add_pentadecathlon(framebuffer, x, y),
        Organism::LWSS => add_lwss(framebuffer, x, y),
        Organism::MWSS => add_mwss(framebuffer, x, y),
        Organism::HWSS => add_hwss(framebuffer, x, y),
    }
}

fn add_glider(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    let glider_pattern = [
        (1, 0),
        (2, 1),
        (0, 2),
        (1, 2),
        (2, 2),
    ];
    for &(dx, dy) in glider_pattern.iter() {
        framebuffer.point((x + dx) as isize, (y + dy) as isize);
    }
}

fn add_block(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    if x + 1 < framebuffer.width - 1 && y + 1 < framebuffer.height - 1 {
        framebuffer.point(x as isize, y as isize);
        framebuffer.point((x + 1) as isize, y as isize);
        framebuffer.point(x as isize, (y + 1) as isize);
        framebuffer.point((x + 1) as isize, (y + 1) as isize);
    }
}

fn add_beehive(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    if x + 3 < framebuffer.width - 1 && y + 2 < framebuffer.height - 1 {
        let beehive_pattern = [
            (1, 0),
            (2, 0),
            (0, 1),
            (3, 1),
            (1, 2),
            (2, 2),
        ];
        for &(dx, dy) in beehive_pattern.iter() {
            framebuffer.point((x + dx) as isize, (y + dy) as isize);
        }
    }
}

fn add_loaf(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    if x + 4 < framebuffer.width - 1 && y + 3 < framebuffer.height - 1 {
        let loaf_pattern = [
            (1, 0),
            (2, 0),
            (0, 1),
            (3, 1),
            (0, 2),
            (3, 2),
            (1, 3),
            (2, 3),
        ];
        for &(dx, dy) in loaf_pattern.iter() {
            framebuffer.point((x + dx) as isize, (y + dy) as isize);
        }
    }
}

fn add_boat(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    if x + 3 < framebuffer.width - 1 && y + 2 < framebuffer.height - 1 {
        let boat_pattern = [
            (1, 0),
            (2, 0),
            (0, 1),
            (1, 1),
            (0, 2),
            (1, 2),
        ];
        for &(dx, dy) in boat_pattern.iter() {
            framebuffer.point((x + dx) as isize, (y + dy) as isize);
        }
    }
}

fn add_tub(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    if x + 2 < framebuffer.width - 1 && y + 2 < framebuffer.height - 1 {
        let tub_pattern = [
            (1, 0),
            (2, 0),
            (0, 1),
            (2, 1),
            (1, 2),
        ];
        for &(dx, dy) in tub_pattern.iter() {
            framebuffer.point((x + dx) as isize, (y + dy) as isize);
        }
    }
}

fn add_blinker(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    if x + 2 < framebuffer.width - 1 && y + 1 < framebuffer.height - 1 {
        let blinker_pattern = [
            (1, 0),
            (0, 1),
            (1, 1),
            (2, 1),
        ];
        for &(dx, dy) in blinker_pattern.iter() {
            framebuffer.point((x + dx) as isize, (y + dy) as isize);
        }
    }
}

fn add_toad(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    if x + 3 < framebuffer.width - 1 && y + 2 < framebuffer.height - 1 {
        let toad_pattern = [
            (0, 1),
            (1, 1),
            (2, 1),
            (1, 0),
            (2, 0),
            (3, 0),
        ];
        for &(dx, dy) in toad_pattern.iter() {
            framebuffer.point((x + dx) as isize, (y + dy) as isize);
        }
    }
}

fn add_beacon(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    if x + 3 < framebuffer.width - 1 && y + 3 < framebuffer.height - 1 {
        let beacon_pattern = [
            (0, 0),
            (1, 0),
            (0, 1),
            (1, 1),
            (2, 2),
            (2, 3),
            (3, 2),
            (3, 3),
        ];
        for &(dx, dy) in beacon_pattern.iter() {
            framebuffer.point((x + dx) as isize, (y + dy) as isize);
        }
    }
}

fn add_pulsar(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    // Example implementation, customize as needed
    if x + 13 < framebuffer.width - 1 && y + 13 < framebuffer.height - 1 {
        let pulsar_pattern = [
            (0, 2),
            (0, 3),
            (1, 0),
            (1, 4),
            (2, 0),
            (2, 4),
            (3, 2),
            (3, 3),
            (4, 0),
            (4, 4),
        ];
        for &(dx, dy) in pulsar_pattern.iter() {
            framebuffer.point((x + dx) as isize, (y + dy) as isize);
        }
    }
}

fn add_pentadecathlon(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    // Example implementation, customize as needed
    if x + 15 < framebuffer.width - 1 && y + 15 < framebuffer.height - 1 {
        let pentadecathlon_pattern = [
            (0, 2),
            (1, 1),
            (1, 2),
            (1, 3),
            (1, 4),
            (2, 0),
            (2, 5),
            (3, 0),
            (3, 5),
            (4, 1),
            (4, 2),
            (4, 3),
            (4, 4),
            (5, 2),
            (6, 2),
        ];
        for &(dx, dy) in pentadecathlon_pattern.iter() {
            framebuffer.point((x + dx) as isize, (y + dy) as isize);
        }
    }
}

fn add_lwss(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    // Example implementation, customize as needed
    if x + 6 < framebuffer.width - 1 && y + 2 < framebuffer.height - 1 {
        let lwss_pattern = [
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (0, 1),
            (4, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
        ];
        for &(dx, dy) in lwss_pattern.iter() {
            framebuffer.point((x + dx) as isize, (y + dy) as isize);
        }
    }
}

fn add_mwss(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    // Example implementation, customize as needed
    if x + 7 < framebuffer.width - 1 && y + 3 < framebuffer.height - 1 {
        let mwss_pattern = [
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0),
            (0, 1),
            (6, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (6, 2),
            (0, 3),
            (6, 3),
        ];
        for &(dx, dy) in mwss_pattern.iter() {
            framebuffer.point((x + dx) as isize, (y + dy) as isize);
        }
    }
}

fn add_hwss(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    // Example implementation, customize as needed
    if x + 8 < framebuffer.width - 1 && y + 4 < framebuffer.height - 1 {
        let hwss_pattern = [
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 0),
            (7, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 1),
            (5, 1),
            (6, 1),
            (7, 1),
            (0, 2),
            (7, 2),
            (0, 3),
            (7, 3),
            (0, 4),
            (1, 4),
            (2, 4),
            (3, 4),
            (4, 4),
            (5, 4),
            (6, 4),
            (7, 4),
        ];
        for &(dx, dy) in hwss_pattern.iter() {
            framebuffer.point((x + dx) as isize, (y + dy) as isize);
        }
    }
}

fn count_live_neighbors(framebuffer: &Framebuffer, x: isize, y: isize) -> usize {
    let mut count = 0;
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),          (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    for &(dx, dy) in directions.iter() {
        let nx = x + dx;
        let ny = y + dy;
        if nx >= 0 && nx < framebuffer.width as isize &&
           ny >= 0 && ny < framebuffer.height as isize {
            let index = (ny as usize) * framebuffer.width + (nx as usize);
            if framebuffer.buffer[index] != framebuffer.background_color {
                count += 1;
            }
        }
    }

    count
}

fn update_framebuffer(framebuffer: &mut Framebuffer) {
    let mut new_buffer = framebuffer.buffer.clone();
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let index = y * framebuffer.width + x;
            let is_alive = framebuffer.buffer[index] != framebuffer.background_color;
            let live_neighbors = count_live_neighbors(framebuffer, x as isize, y as isize);

            if is_alive {
                if live_neighbors < 2 || live_neighbors > 3 {
                    new_buffer[index] = framebuffer.background_color; // Muere
                }
            } else {
                if live_neighbors == 3 {
                    new_buffer[index] = 0xFFFFFFFF; // Nace
                }
            }
        }
    }

    framebuffer.buffer = new_buffer;
}

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);
    initialize_pattern(&mut framebuffer, 50);

    let mut window = Window::new("Conway's Game of Life", width, height, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("Window creation failed: {:?}", e);
        });

    window.limit_update_rate(Some(Duration::from_millis(100))); // Puedes cambiar esto si usas set_fps_target

    while window.is_open() && !window.is_key_down(Key::Escape) {
        update_framebuffer(&mut framebuffer);
        window.update_with_buffer(&framebuffer.buffer, width, height).unwrap();
    }
}