
pub fn run() {
    println!("Hello, world!");

    println!("{:#?}", two_sum(vec![1,2,3], 4));
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res = vec![-1;2];

    for i in 0..nums.len() {
        let a = target - nums[i];

        for j in (i + 1)..nums.len() {
            if  a == nums[j] {
                res[0] = i as i32;
                res[1] = j as i32;
                return  res;
            }
        }
    }
    res
}