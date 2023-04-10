use std::cmp;

use crate::{EditScript, EditType, Options};

pub trait Matrix {
    fn set(&mut self, x: usize, y: usize, value: usize);

    fn get(&self, x: usize, y: usize) -> usize;

    fn new(x: usize, y: usize) -> Self;
}

pub struct Osa<T>
where
    T: Matrix,
{
    pub matrix: T,
    source: Vec<char>,
    target: Vec<char>,
    options: Options,
}

impl<T> Osa<T>
where
    T: Matrix,
{
    pub fn new(source: &str, target: &str, options: Options, matrix: T) -> Osa<T>
    where
        T: Matrix,
    {
        let source: Vec<char> = source.chars().collect();
        let target: Vec<char> = target.chars().collect();

        return Osa {
            matrix,
            source,
            target,
            options,
        };
    }
    pub fn get_matrix(&self) -> &T {
        return &self.matrix;
    }
    pub fn edit_script_for_strings(&mut self) -> EditScript {
        self.matrix_for_strings();
        return self.backtrace(self.source.len(), self.target.len());
    }

    pub fn matrix_for_strings(&mut self) {
        let height = self.source.len() + 1;
        let width = self.target.len() + 1;

        for i in 0..height {
            self.matrix.set(i, 0, i * self.options.del_cost);
        }
        for j in 0..width {
            self.matrix.set(0, j, j * self.options.ins_cost);
        }
        for i in 1..height {
            for j in 1..width {
                let del_cost = self.matrix.get(i - 1, j) + self.options.del_cost;
                let mut match_sub_cost = self.matrix.get(i - 1, j - 1);
                if !(self.options.equals)(self.source[(i - 1)], self.target[j - 1]) {
                    match_sub_cost += self.options.sub_cost;
                }
                let ins_cost = self.matrix.get(i, j - 1) + self.options.ins_cost;
                self.matrix
                    .set(i, j, cmp::min(del_cost, cmp::min(match_sub_cost, ins_cost)));
                if i > 1
                    && j > 1
                    && self.source[i - 1] == self.target[j - 2]
                    && self.source[i - 2] == self.target[j - 1]
                {
                    let transp_cost = self.matrix.get(i - 2, j - 2) + self.options.transp_cost;
                    self.matrix
                        .set(i, j, cmp::min(self.matrix.get(i, j), transp_cost));
                }
            }
        }
    }

    fn backtrace(&self, i: usize, j: usize) -> EditScript {
        if i > 1
            && j > 1
            && self.source[i - 1] == self.target[j - 2]
            && self.source[i - 2] == self.target[j - 1]
        {
            if self.matrix.get(i - 2, j - 2) < self.matrix.get(i, j) {
                let mut v = self.backtrace(i - 2, j - 2);
                v.push(EditType::Transpose);
                v.push(EditType::Transpose);
                return v;
            }
        }
        if i > 0 && self.matrix.get(i - 1, j) + self.options.del_cost == self.matrix.get(i, j) {
            let mut v = self.backtrace(i - 1, j);
            v.push(EditType::Delete);
            return v;
        }
        if j > 0 && self.matrix.get(i, j - 1) + self.options.ins_cost == self.matrix.get(i, j) {
            let mut v = self.backtrace(i, j - 1);
            v.push(EditType::Insert);
            return v;
        }
        if i > 0
            && j > 0
            && self.matrix.get(i - 1, j - 1) + self.options.sub_cost == self.matrix.get(i, j)
        {
            let mut v = self.backtrace(i - 1, j - 1);
            v.push(EditType::Sub);
            return v;
        }
        if i > 0 && j > 0 && self.matrix.get(i - 1, j - 1) == self.matrix.get(i, j) {
            let mut v = self.backtrace(i - 1, j - 1);
            v.push(EditType::Equal);
            return v;
        }
        return vec![];
    }
}
