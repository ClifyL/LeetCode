pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left:i32 = 0;
    let mut right = (nums.len() - 1) as i32;
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid as usize] == target {
            return mid;
        } else if nums[mid as usize] > target {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    -1
}

fn main() {
    let result = search(vec![5], 4);
    println!("{result}"); 
}

