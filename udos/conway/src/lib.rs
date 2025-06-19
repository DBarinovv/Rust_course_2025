#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Grid {
            rows: rows,
            cols: cols,
            grid: vec![T::default(); rows * cols],
        }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        Grid {
            rows: rows,
            cols: cols,
            grid: grid.to_vec(),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &(self.grid[row * self.cols + col])
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        self.grid[row * self.cols + col] = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {

        let mut result = Vec::new();
    
        for i in -1..=1 {
            for j in -1..=1 {
                if i != 0 || j != 0 {

                    let (x, y) = (row as isize + i, col as isize + j);

                    if x >= 0 && x < self.rows as isize 
                    && y >= 0 && y < self.cols as isize {
                        result.push((x as usize, y as usize));
                    }
                }
            }
        }
        
        result
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        GameOfLife { grid }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    pub fn step(&mut self) {
        let (rows, cols) = self.grid.size();
        let mut next = Grid::new(rows, cols);

        for i in 0..rows {
            for j in 0..cols {

                let neighbour = self.grid.neighbours(i, j);

                let mut cnt = 0;

                for &(x, y) in &neighbour {
                    cnt += (self.grid.get(x, y) == &Cell::Alive) as usize;
                }
                
                let next_point;

                if cnt == 3 || (self.grid.get(i, j) == &Cell::Alive && cnt == 2) {
                    next_point = Cell::Alive;
                } else {
                    next_point = Cell::Dead;
                }

                next.set(next_point, i, j);
            }
        }
        self.grid = next;
    }
}
