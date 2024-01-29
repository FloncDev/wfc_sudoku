pub struct Sudoku {
    n: u32,
    grid: Vec<Vec<Option<u32>>>,
}

#[derive(Debug)]
pub enum ValidationError {
    OutOfGrid,
    RegionHasSameNumber(u32, u32),
    GroupHasSameNumber(u32, u32),
}

impl Sudoku {
    // n is the size of a square, traditionally is 3
    pub fn new(n: u32) -> Self {
        let grid = vec![vec![None; n.pow(2) as usize]; n.pow(2) as usize];

        Sudoku { n, grid }
    }

    // (0, 0) is top right and (n^2-1, n^2-1) is bottom right
    pub fn is_valid(&self, coords: (u32, u32), num: u32) -> Result<bool, ValidationError> {
        let (x, y) = coords;

        // Max number is n^2
        if x >= self.n.pow(2) && y >= self.n.pow(2) {
            return Err(ValidationError::OutOfGrid);
        };

        // Check same region
        // May seem weird to divide then multiply by n but it finds
        // the top left cell of the square as u32 / u32 floors the
        // numbers
        let (region_x, region_y) = (x / self.n, y / self.n);
        let (corner_x, corner_y) = (region_x * self.n, region_y * self.n);

        for x_offset in 0..self.n {
            for y_offset in 0..self.n {
                let new_coords = (corner_x + x_offset, corner_y + y_offset);

                // Cell might already contain value and it should be ignored
                if new_coords == coords {
                    continue;
                }

                if self.get(new_coords) == Some(num) {
                    return Err(ValidationError::RegionHasSameNumber(
                        new_coords.0,
                        new_coords.1,
                    ));
                }
            }
        }

        // Check rows and columns (groups)
        // Couldn't find a way to do this cleaner
        for (new_x, i) in self.grid[y as usize].iter().enumerate() {
            if new_x as u32 == x {
                continue;
            }

            if i.is_none() {
                continue;
            }

            if i.unwrap() == num {
                return Err(ValidationError::GroupHasSameNumber(new_x as u32, y));
            }
        }

        for (new_y, i) in self.grid.iter().map(|row| row[x as usize]).enumerate() {
            if new_y as u32 == y {
                continue;
            }

            if i.is_none() {
                continue;
            }

            if i.unwrap() == num {
                return Err(ValidationError::GroupHasSameNumber(x, new_y as u32));
            }
        }

        Ok(true)
    }

    pub fn get(&self, coords: (u32, u32)) -> Option<u32> {
        self.grid[coords.1 as usize][coords.0 as usize]
    }

    pub fn set(&mut self, coords: (u32, u32), num: u32) -> Result<(), ValidationError> {
        if let Err(err) = self.is_valid(coords, num) {
            return Err(err);
        }

        self.grid[coords.1 as usize][coords.0 as usize] = Some(num);
        Ok(())
    }
}
