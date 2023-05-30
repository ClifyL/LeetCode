use std::cmp::Reverse;

pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
    let mut envelopes = envelopes;
    envelopes.sort_unstable_by_key(|k| (k[0], Reverse(k[1])));
    let nums: Vec<i32> = envelopes.iter().map(|a| a[1]).collect();
    let mut dp = vec![1; nums.len()];
    let mut res: i32 = 0;
    for i in 0..nums.len() {
        for j in 0..i {
            if nums[i] > nums[j] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
        res = res.max(dp[i] as i32);
    }
    res
}


fn main() {
    let count = max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]]);
    println!("{count}");
}
