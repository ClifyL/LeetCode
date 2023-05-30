
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut dp = vec![1 ; nums.len()];
    let mut res:i32 = 0;
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
    let count = length_of_lis(vec![1, 2, 3, 4]);
    println!("{count}");
}
