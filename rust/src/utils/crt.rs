use grid::*;

pub struct Crt {
    grid: Grid<char>,
    width: usize,
    location: usize,
}

impl Crt {
    pub fn new(width: usize, height: usize) -> Crt {
        Crt {
            grid: Grid::new(height, width),
            width,
            location: 0,
        }
    }

    fn set_pixel(&mut self, at: usize, value: char) {
        let x = at % self.width;
        let y = at / self.width;
        self.grid[y][x] = value;
    }

    pub fn to_string(&self) -> String {
        self.grid.iter().fold("".to_string(), |mut a, c| {
            a.push(*c);
            a
        })
    }

    fn print_current_row(&self) {
        let start = self.location / self.width;
        let word = self.to_string();
        let part = &word[start..start + self.width];
        log::debug!("Current CRT row: {}", part);
    }

    pub fn print_sprite(&self, sprite: usize) {
        let mut draw_sprite = vec!['.'; self.width];
        if sprite > 0 && sprite < 39 {
            draw_sprite[sprite - 1] = '#';
            draw_sprite[sprite] = '#';
            draw_sprite[sprite + 1] = '#';
        }
        log::debug!(
            "Sprite position: {}\n",
            draw_sprite.into_iter().collect::<String>()
        );
    }

    pub fn step(&mut self, sprite: i32) {
        let row_clock: i32 = self.location as i32 % self.width as i32;
        let pixel = match self.location {
            _ if (sprite - 1..=sprite + 1).contains(&row_clock) => '#',
            _ => '.',
        };

        log::debug!(
            "During cycle{:03}: CRT draws pixel in position {} {}",
            self.location + 1,
            self.location,
            sprite,
        );

        self.set_pixel(self.location, pixel);
        self.print_current_row();

        self.location += 1;
    }
}
