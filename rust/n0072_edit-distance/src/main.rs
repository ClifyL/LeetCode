pub fn min_distance(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 0..m + 1 {
        dp[i][0] = i;
    }
    for j in 0..n + 1 {
        dp[0][j] = j;
    }
    for (i, s1) in word1.chars().enumerate() {
        for (j, s2) in word2.chars().enumerate() {
            if s1 == s2 {
                dp[i + 1][j + 1] = dp[i][j];
            } else {
                dp[i + 1][j + 1] = std::cmp::min(
                    dp[i + 1][j] + 1, // 插入
                    std::cmp::min(
                        dp[i][j + 1] + 1, // 删除
                        dp[i][j] + 1, // 替换
                    ),
                );
            }
        }
    }
    dp[m][n] as i32
}    
// 递归常规解法
pub fn min_distance_rec(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();
    fn dp(s1: &str, i: i32, s2: &str, j: i32) -> i32 {
        if i == -1 {
            return j + 1;
        }
        if j == -1 {
            return i + 1;
        }
    
        if s1.chars().nth(i as usize).unwrap() == s2.chars().nth(j as usize).unwrap() {
            return dp(s1, i - 1, s2, j - 1);
        }
        return std::cmp::min(
            dp(s1, i, s2, j - 1) + 1, // 插入
            std::cmp::min(
                dp(s1, i - 1, s2, j) + 1, // 删除
                dp(s1, i - 1, s2, j - 1) + 1, // 替换
            ),
        );
    }
    // i，j 初始化指向最后一个索引
    dp(&word1[0..], m as i32 - 1, &word2[0..], n as i32 - 1)
}



fn main() {
    let source_word = String::from("horse");
    let target_word = String::from("ros");
    let distance = min_distance(source_word, target_word);
    println!("{distance}");
}
