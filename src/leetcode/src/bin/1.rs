use std::collections::HashMap;
fn main() {
    let s = vec![2, 7, 11, 15];
    let res = two_sum(s, 9);
    println!("{:?}", res)
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut cache = HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        let ext = target - v;
        if let Some(v) = cache.get(v) {
            return vec![*v as i32, i as i32];
        }
        cache.insert(ext, i);
    }
    vec![]
}

