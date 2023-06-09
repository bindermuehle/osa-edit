use super::osa::Matrix;

#[derive(Debug, Clone)]
pub struct VecMatrix {
    pub matrix: Vec<Vec<usize>>,
}

impl Matrix for VecMatrix {
    fn set(&mut self, x: usize, y: usize, value: usize) {
        self.matrix[x][y] = value;
    }
    fn get(&self, x: usize, y: usize) -> usize {
        self.matrix[x][y]
    }
    fn new(x: usize, y: usize) -> VecMatrix {
        return VecMatrix {
            matrix: vec![vec![0; y]; x],
        };
    }
    fn get_last_cell(&self) -> usize {
        let x = self.matrix.len() - 1;
        let y = self.matrix[0].len() - 1;
        return self.matrix[x][y];
    }
}
