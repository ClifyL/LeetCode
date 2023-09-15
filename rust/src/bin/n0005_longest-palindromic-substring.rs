// dp
pub fn longest_palindrome(s: String) -> String {
    let n = s.len();
    let mut head: usize = 0;
    let mut length = 1;
    let mut dp = vec![vec![0 ; n] ; n];
    let t: Vec<char> = s.chars().collect();
    for i in 0..n {
        dp[i][i] = 1;
    }

    for l in 2..n + 1 {
        for i in 0..n {
            if  i + l - 1 > n - 1 {
                break;
            }
            if t[i] == t[i + l - 1] {
                if l < 4 {
                    dp[i][i + l - 1] = 1;
                } else {
                    dp[i][i + l - 1] = dp[i + 1][i + l - 2];
                }
            }
            if dp[i][i + l - 1] == 1 && l > length {
                length = l;
                head = i;
            }
        }
    }

    s[head..(head + length)].to_string()
}

fn main() {
    let s = String::from("abba");
    let result = longest_palindrome(s);
    println!("{result}");
}
