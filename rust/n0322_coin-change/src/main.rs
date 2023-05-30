pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp = vec![amount + 1; (amount + 1) as usize];
    dp[0] = 0;
    // 自底向上寻找最优解
    for i in 0..dp.len() {
        for &coin in coins.iter() {
            if i as i32 - coin < 0 {
                continue
            }
            dp[i] = dp[i].min(1 + dp[i - coin as usize]);
        }
    }
    if dp[amount as usize] == amount + 1 {
        -1
    } else {
        dp[amount as usize]
    }
}

fn main() {
    let count = coin_change(vec![1, 2, 10], 20);
    println!("{count}");
}
