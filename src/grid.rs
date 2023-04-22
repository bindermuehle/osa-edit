use grid::Grid;

use crate::osa::Matrix;

pub struct GridMatrix {
    pub matrix: Grid<usize>,
}

impl Matrix for GridMatrix {
    fn set(&mut self, x: usize, y: usize, value: usize) {
        self.matrix[x][y] = value;
    }
    fn get(&self, x: usize, y: usize) -> usize {
        self.matrix[x][y]
    }
    fn new(x: usize, y: usize) -> GridMatrix {
        return GridMatrix {
            matrix: Grid::new(x, y),
        };
    }
    fn get_last_cell(&self) -> usize {
        return self.matrix[self.matrix.rows() - 1][self.matrix.cols() - 1];
    }
}
