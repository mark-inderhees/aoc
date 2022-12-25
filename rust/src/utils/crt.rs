use grid::*;

/// A CRT screen display. It draws a pixel if the user set sprite location
/// overlaps the internal timer.
/// The sprite is 3 units wide but could be made a configurable width.
/// The timer scans the whole display one row at a time. The user set sprite
/// location is only the width of a single row, which will match whatever row
/// the timer currently is on.
/// A pixel is either on or off but could be expanded to have more states (like color).
pub struct Crt {
    grid: Grid<char>,
    width: usize,
    time: usize,
}

impl Crt {
    /// Create a new CRT. Time starts at zero.
    pub fn new(width: usize, height: usize) -> Crt {
        Crt {
            grid: Grid::new(height, width),
            width,
            time: 0,
        }
    }

    /// Set a pixel to a certain value. The at value is the current time, it
    /// scans over the full grid before wrapping around.
    fn set_pixel(&mut self, at: usize, value: char) {
        let x = at % self.width;
        let y = at / self.width;
        self.grid[y][x] = value;
    }

    /// Print the grid value as one long string.
    pub fn to_string(&self) -> String {
        self.grid.iter().fold("".to_string(), |mut a, c| {
            a.push(*c);
            a
        })
    }

    /// Print just the current row that the CRT is scanning based on the current time.
    fn print_current_row(&self) {
        let start = self.time / self.width;
        let word = self.to_string();
        let part = &word[start..start + self.width];
        log::debug!("Current CRT row: {}", part);
    }

    /// Based on the user set sprite, print where the sprite location is.
    pub fn print_sprite(&self, sprite: usize) {
        let mut draw_sprite = vec!['.'; self.width];
        if sprite > 0 && sprite < self.width - 1 {
            draw_sprite[sprite - 1] = '#';
            draw_sprite[sprite] = '#';
            draw_sprite[sprite + 1] = '#';
        }
        log::debug!(
            "Sprite position: {}\n",
            draw_sprite.into_iter().collect::<String>()
        );
    }

    /// Increment the internal timer of the CRT and set the value of the pixel
    /// based on the location of the user set sprite.
    pub fn step(&mut self, sprite: i32) {
        // Internal time scans the whole grid, but sprite location is for the row,
        // so convert internal time to where it is within one row.
        let row_clock: i32 = self.time as i32 % self.width as i32;

        // If the row time overlaps the sprite, then light up the pixel
        let pixel = match self.time {
            _ if (sprite - 1..=sprite + 1).contains(&row_clock) => '#',
            _ => '.',
        };

        log::debug!(
            "During cycle{:03}: CRT draws pixel in position {} {}",
            self.time + 1,
            self.time,
            sprite,
        );

        self.set_pixel(self.time, pixel);
        self.print_current_row();

        self.time += 1;
    }
}
