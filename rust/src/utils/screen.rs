use grid::*;
use rusttype::Point;

// A screen to display pixels. Supports setting blocks of pixels and shifting
// pixeles by rows or columns. And can print them to the console for easy debug.
pub struct Screen {
    grid: Grid<char>,
}

impl Screen {
    /// Create a new Screen.
    pub fn new(width: usize, height: usize) -> Screen {
        let mut screen = Screen {
            grid: Grid::new(height, width),
        };

        // Init all pixels to off as '.', where '#' where be on
        for x in 0..width {
            for y in 0..height {
                screen.grid[y][x] = '.';
            }
        }
        screen
    }

    /// Turn pixels on at a location, with a given width and height
    pub fn set_pixels(&mut self, at: Point<usize>, width: usize, height: usize) {
        log::debug!("Set pixels {}x{} at ({},{})", width, height, at.x, at.y);
        for x in at.x..at.x + width {
            for y in at.y..at.y + height {
                self.grid[y][x] = '#';
            }
        }
        self.debug_print();
    }

    // Rotate the pixels in a row a fixed number of times. Pixels rotate back
    // to the begining of the row.
    pub fn rotate_row(&mut self, row: usize, count: usize) {
        log::debug!("Rotate row {} by {} pixels", row, count);
        let orig: Vec<char> = self.grid.iter_row(row).map(|x| *x).collect();
        let width = self.grid.cols();
        let offset = count % width;
        for i in 0..width {
            self.grid[row][(i + offset) % width] = orig[i];
        }
        self.debug_print();
    }

    // Rotate the pixels in a column a fixed number of times. Pixels rotate back
    // to the begining of the column.
    pub fn rotate_column(&mut self, column: usize, count: usize) {
        log::debug!("Rotate column {} by {} pixels", column, count);
        let orig: Vec<char> = self.grid.iter_col(column).map(|x| *x).collect();
        let height = self.grid.rows();
        let offset = count % height;
        for i in 0..height {
            self.grid[(i + offset) % height][column] = orig[i];
        }
        self.debug_print();
    }

    /// Convert pixels to a string for easy display
    pub fn to_string(&self) -> String {
        let mut string = "\n".to_string();
        for row in 0..self.grid.rows() {
            string += &self.grid.iter_row(row).map(|x| *x).collect::<String>();
            string += "\n";
        }
        string
    }

    /// Print the grid values.
    pub fn debug_print(&self) {
        let string = self.to_string();
        log::debug!("{}", string);
    }

    // Count how many pixels are on
    pub fn count_set_pixels(&self) -> usize {
        let mut count = 0;
        for x in 0..self.grid.cols() {
            for y in 0..self.grid.rows() {
                if self.grid[y][x] == '#' {
                    count += 1;
                }
            }
        }
        count
    }
}
