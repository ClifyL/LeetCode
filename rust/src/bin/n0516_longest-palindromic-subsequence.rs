// 2D dp
/*
        i -> n
       b b b a b
     b 
     b
j->n b
     a
     b

    The state-transition equation is: 
        f[i][j] = f[i+1]f[j-1] + 2 (s[j] == s[i])
        f[i][j] = Max(f[i+1][j], f[i][j-1]) (s[j] != s[i])


*/
pub fn longest_palindrome_subseq_2d(s: String) -> i32 {
    let n = s.len();
    let mut dp = vec![vec![0; n]; n];
    let t: Vec<char> = s.chars().collect();
    for i in (0..n-1).rev() {
        dp[i][i] = 1;
        for j in i + 1..n {
            if &t[i] == &t[j] {
                dp[i][j] = dp[i + 1][j - 1] + 2;
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp[0][n - 1]
}

pub fn longest_palindrome_subseq(s: String) -> i32 {
    let n = s.len();
    let mut dp = vec![1; n];
    let t: Vec<char> = s.chars().collect();
    let mut pre : i32;
    for i in (0..n-1).rev() {
        pre = 0;
        for j in i + 1..n {
            let temp = dp[j];
            if &t[i] == &t[j] {
                dp[j] = pre + 2;
            } else {
                dp[j] = dp[j].max(dp[j - 1]);
            }
            pre = temp;
        }
    }
    dp[n - 1]
}

fn main() {
    let result = longest_palindrome_subseq_2d("bab".to_string());
    println!("{result}");
}
