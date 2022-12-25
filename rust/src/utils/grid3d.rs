/// A 3D grid. This differs from Board3d, which is a cube shaped game board.
/// This is a true 3d map, all internal locations are accessible.
pub struct Grid3d<T> {
    grid: Vec<Vec<Vec<T>>>,
    size: usize,
}

impl<T> Grid3d<T>
where
    T: Clone,
{
    /// Create a new 3D grid of a fixed size. Only cube is supported.
    pub fn new(size: usize, init_value: T) -> Grid3d<T> {
        Grid3d {
            grid: vec![vec![vec![init_value; size]; size]; size],
            size,
        }
    }

    /// Set the value at this location.
    pub fn set_at(&mut self, x: usize, y: usize, z: usize, value: T) {
        self.grid[x][y][z] = value;
    }

    /// Get the value at this location.
    pub fn get_at(&self, x: usize, y: usize, z: usize) -> T {
        self.grid[x][y][z].clone()
    }

    /// Get the size of the cube, this is single x, y or z length.
    pub fn get_size(&self) -> usize {
        self.size
    }

    /// Resize the grid down to this size
    pub fn resize(&mut self, size: usize) {
        assert!(size < self.size);
        for _ in size..self.size {
            self.grid.pop();
        }
        for x in 0..size {
            for _ in size..self.size {
                self.grid[x].pop();
            }
        }
        for x in 0..size {
            for y in 0..size {
                for _ in size..self.size {
                    self.grid[x][y].pop();
                }
            }
        }

        self.size = size;
    }
}
