pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut dp = vec![false; s.len() + 1];
    dp[0] = true;
    for i in 1..s.len() + 1 {
        for word in &word_dict {
            if word.len() > i {
                continue;
            }
            if dp[i - word.len()] && &s[i - word.len()..i] == word {
                dp[i] = true;
                break;
            }
        }
    }
    dp[s.len()]
}

fn main() {
    let result = word_break(
        "leetcode".to_string(),
        vec!["leet".to_string(), "code".to_string()],
    );
    println!("{result}");
}
