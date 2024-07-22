pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub current_color: u32,
    pub background_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
            current_color: 0xFFFFFFFF,
            background_color: 0x00000000,
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn clear(&mut self) {
        self.buffer.iter_mut().for_each(|pixel| *pixel = self.background_color);
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            self.buffer[(y as usize) * self.width + (x as usize)] = self.current_color;
        }
    }

    pub fn get_color(&self, x: isize, y: isize) -> Option<u32> {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            Some(self.buffer[(y as usize) * self.width + (x as usize)])
        } else {
            None
        }
    }
}
