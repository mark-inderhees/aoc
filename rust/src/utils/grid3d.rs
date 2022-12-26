/// A 3D grid. This differs from Board3d, which is a cube shaped game board.
/// This is a true 3d map, all internal locations are accessible.
pub struct Grid3d<T> {
    grid: Vec<Vec<Vec<T>>>,
    size: usize,
}

#[derive(Debug, Clone)]
pub struct Point3d {
    pub x: usize,
    pub y: usize,
    pub z: usize,
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
    pub fn set_at(&mut self, point: &Point3d, value: T) {
        self.grid[point.x][point.y][point.z] = value;
    }

    /// Get the value at this location.
    pub fn value_at(&self, point: &Point3d) -> T {
        self.grid[point.x][point.y][point.z].clone()
    }

    /// Get the size of the cube, this is single x, y or z length.
    pub fn size(&self) -> usize {
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

    /// Return a list of all nearby values. This only includes straight moves,
    /// not diagonal.
    pub fn nearby_values(&self, point: &Point3d) -> Vec<T> {
        let mut values = vec![];

        let size_signed = self.size as isize;
        let x = point.x as isize;
        let y = point.y as isize;
        let z = point.z as isize;
        let x_range = [-1, 1, 0, 0, 0, 0];
        let y_range = [0, 0, -1, 1, 0, 0];
        let z_range = [0, 0, 0, 0, -1, 1];
        for ((dx, dy), dz) in x_range.iter().zip(y_range.iter()).zip(z_range.iter()) {
            let x = x + dx;
            let y = y + dy;
            let z = z + dz;
            if x < 0 || x >= size_signed {
                continue;
            }
            if y < 0 || y >= size_signed {
                continue;
            }
            if z < 0 || z >= size_signed {
                continue;
            }
            values.push(self.value_at(&Point3d {
                x: x as usize,
                y: y as usize,
                z: z as usize,
            }));
        }

        values
    }
}
