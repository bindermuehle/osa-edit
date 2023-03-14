use std::cmp;

#[derive(Debug, PartialEq)]
pub enum EditType {
    Insert,
    Delete,
    Sub,
    Equal,
    Transpose,
}

pub type EditScript = Vec<EditType>;

type MatchFunction = fn(char, char) -> bool;

fn identical_chars(a: char, b: char) -> bool {
    return a == b;
}

#[derive(Copy, Clone)]
pub struct Options {
    ins_cost: usize,
    del_cost: usize,
    sub_cost: usize,
    transp_cost: usize,
    equals: MatchFunction,
}

pub const DEFAULT_OPTIONS: Options = Options {
    ins_cost: 1,
    del_cost: 1,
    sub_cost: 1,
    transp_cost: 1,
    equals: identical_chars,
};

pub const DEFAULT_OPTIONS_WITH_SUB: Options = Options {
    ins_cost: 1,
    del_cost: 1,
    sub_cost: 1,
    transp_cost: 1,
    equals: identical_chars,
};
pub fn edit_script_for_strings(source: &str, target: &str, op: Options) -> EditScript {
    let source: Vec<char> = source.chars().collect();
    let target: Vec<char> = target.chars().collect();
    return backtrace(
        source.len(),
        target.len(),
        &source,
        &target,
        matrix_for_strings(&source, &target, op),
        op,
    );
}

pub fn matrix_for_strings(source: &Vec<char>, target: &Vec<char>, op: Options) -> Vec<Vec<usize>> {
    let height = source.len() + 1;
    let width = target.len() + 1;

    let mut matrix = vec![vec![0; width]; height];
    for i in 0..height {
        matrix[i][0] = i * op.del_cost
    }
    for j in 0..width {
        matrix[0][j] = j * op.ins_cost
    }
    for i in 1..height {
        for j in 1..width {
            let del_cost = matrix[i - 1][j] + op.del_cost;
            let mut match_sub_cost = matrix[i - 1][j - 1];
            if !(op.equals)(source[(i - 1)], target[j - 1]) {
                match_sub_cost += op.sub_cost;
            }
            let ins_cost = matrix[i][j - 1] + op.ins_cost;
            matrix[i][j] = cmp::min(del_cost, cmp::min(match_sub_cost, ins_cost));
            if i > 1 && j > 1 && source[i - 1] == target[j - 2] && source[i - 2] == target[j - 1] {
                let transp_cost = matrix[i - 2][j - 2] + op.transp_cost;
                matrix[i][j] = cmp::min(matrix[i][j], transp_cost)
            }
        }
    }
    return matrix;
}

fn backtrace(
    i: usize,
    j: usize,
    source: &Vec<char>,
    target: &Vec<char>,
    matrix: Vec<Vec<usize>>,
    op: Options,
) -> EditScript {
    if i > 1 && j > 1 && source[i - 1] == target[j - 2] && source[i - 2] == target[j - 1] {
        if matrix[i - 2][j - 2] < matrix[i][j] {
            let mut v = backtrace(i - 2, j - 2, source, target, matrix, op);
            v.push(EditType::Transpose);
            v.push(EditType::Transpose);
            return v;
        }
    }
    if i > 0 && matrix[i - 1][j] + op.del_cost == matrix[i][j] {
        let mut v = backtrace(i - 1, j, source, target, matrix, op);
        v.push(EditType::Delete);
        return v;
    }
    if j > 0 && matrix[i][j - 1] + op.ins_cost == matrix[i][j] {
        let mut v = backtrace(i, j - 1, source, target, matrix, op);
        v.push(EditType::Insert);
        return v;
    }
    if i > 0 && j > 0 && matrix[i - 1][j - 1] + op.sub_cost == matrix[i][j] {
        let mut v = backtrace(i - 1, j - 1, source, target, matrix, op);
        v.push(EditType::Sub);
        return v;
    }
    if i > 0 && j > 0 && matrix[i - 1][j - 1] == matrix[i][j] {
        let mut v = backtrace(i - 1, j - 1, source, target, matrix, op);
        v.push(EditType::Equal);
        return v;
    }
    return vec![];
}

#[cfg(test)]
mod tests {
    use super::EditType::{Delete, Equal, Insert, Transpose};
    use super::*;
    struct TestString {
        source: String,
        target: String,
        options: Options,
        //        distance: usize,
        //       ratio: f64,
        script: EditScript,
    }
    fn test_data() -> Vec<TestString> {
        let test_cases: Vec<TestString> = vec![
            TestString {
                source: "".to_string(),
                target: "a".to_string(),
                options: DEFAULT_OPTIONS,
                // distance: 1,
                // ratio: 0.0,
                script: vec![Insert],
            },
            TestString {
                source: "a".to_string(),
                target: "".to_string(),
                options: DEFAULT_OPTIONS,
                // distance: 1,
                // ratio: 0.0,
                script: vec![Delete],
            },
            TestString {
                source: "fish".to_string(),
                target: "ifsh".to_string(),
                options: DEFAULT_OPTIONS,
                script: vec![Transpose, Transpose, Equal, Equal],
            },
            TestString {
                source: "fishy".to_string(),
                target: "ifsh".to_string(),
                options: DEFAULT_OPTIONS,
                script: vec![Transpose, Transpose, Equal, Equal, Delete],
            },
        ];
        return test_cases;
    }
    struct TestMatrix {
        matrix: Vec<Vec<usize>>,
        source: String,
        target: String,
        options: Options,
    }
    fn test_matrix_data() -> Vec<TestMatrix> {
        return vec![TestMatrix {
            matrix: vec![
                vec![0, 1, 2, 3, 4],
                vec![1, 1, 1, 2, 3],
                vec![2, 1, 1, 2, 3],
                vec![3, 2, 2, 1, 2],
                vec![4, 3, 3, 2, 1],
            ],
            source: String::from("fish"),
            target: String::from("ifsh"),
            options: DEFAULT_OPTIONS,
        }];
    }

    #[test]
    fn test_edit_script_for_strings() {
        for t in test_data() {
            let script = edit_script_for_strings(&t.source, &t.target, t.options);
            println!("{:?}", script);
            assert_eq!(t.script.len(), script.len());

            for (i, el) in t.script.iter().enumerate() {
                assert_eq!(*el, script[i], "failed at index {}", i)
            }
        }
    }
    #[test]
    fn test_matrix() {
        for t in test_matrix_data() {
            let result = matrix_for_strings(
                &t.source.chars().collect(),
                &t.target.chars().collect(),
                t.options,
            );
            assert_eq!(result.len(), t.matrix.len());
            for (i, row) in result.iter().enumerate() {
                assert_eq!(row.len(), t.matrix[i].len());
                for (a, b) in row.iter().zip(t.matrix[i].iter()) {
                    assert_eq!(a, b, "values don't match in row {}", i)
                }
            }
        }
    }
}
