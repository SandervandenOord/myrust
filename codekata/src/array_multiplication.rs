pub fn array_multiplication_challenge() {
    let nums: Vec<i32> = vec![3, 5, 6];
    let nums_product = grow(nums);
    println!("{nums_product}");
}

fn grow(nums: Vec<i32>) -> i32 {
    let mut result = nums[0];
    if nums.len() > 1 {
        for num in &nums[1..] {
            result *= num;
        }
    }
    result
}

fn grow_alternative(nums: Vec<i32>) -> i32 {
    nums.iter().product()
}
