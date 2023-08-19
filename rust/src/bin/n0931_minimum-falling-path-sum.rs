// 从n,x到 i,j最小路径可以表示为
// dp[i][j] = matrix[i][j] + min(dp[i + 1][j], dp[i + 1][j - 1], dp[i + 1][j + 1])
pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let mut matrix = matrix;
    let n = matrix.len();
    let mut best;
    for i in (0..n - 1).rev() {
        for j in 0..n {
            best = matrix[i + 1][j];
            if j >= 1 {
                best = best.min(matrix[i + 1][j - 1])
            }
            if j <= n - 2 {
                best = best.min(matrix[i + 1][j + 1])
            }
            matrix[i][j] += best;
        }
    }
    *matrix[0].iter().min().unwrap()
}

// 使用dp函数遍历
fn dp(matrix: &Vec<Vec<i32>>, memory: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    if i >= matrix.len() || j >= matrix[i].len() {
        return 10001;
    }
    if memory[i][j] != 0 {
        return memory[i][j];
    }
    if i == 0 {
        return matrix[i][j];
    }
    if j == 0 {
        memory[i][j] = matrix[i][j]
            + vec![
                dp(matrix, memory, i - 1, j + 1),
                dp(matrix, memory, i - 1, j),
            ]
            .iter()
            .min()
            .unwrap();
    } else {
        memory[i][j] = matrix[i][j]
            + vec![
                dp(matrix, memory, i - 1, j + 1),
                dp(matrix, memory, i - 1, j),
                dp(matrix, memory, i - 1, j - 1),
            ]
            .iter()
            .min()
            .unwrap();
    }
    memory[i][j]
}

fn main() {
    let matrix = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
    let min = min_falling_path_sum(matrix);
    println!("{min}");
}
