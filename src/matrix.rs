use rand::prelude::*;
pub fn create_matrix(rows: i32, column: i32, range: i32) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = vec![];
    let mut col: Vec<i32> = vec![];
    for _e in 0..rows {
        for _i in 0..column {
            let number: i32 = thread_rng().gen_range(1..range);
            col.push(number);
        }
        matrix.push(col);
        col = vec![]
    }
    matrix
}

pub fn sum_matrix(matrix1: &Vec<Vec<i32>>, matrix2: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut matrix = vec![];
    let mut array: Vec<i32> = vec![];
    for i in 0..matrix1.len() {
        for e in 0..matrix1.len() {
            let sum = matrix1[i][e] + matrix2[i][e];
            array.push(sum)
        }
        matrix.push(array);
        array = vec![]
    }
    matrix
}
