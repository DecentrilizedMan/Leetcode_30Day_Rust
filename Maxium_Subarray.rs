// Option 1 
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut s = nums[0];
        let mut max = s;
        for &i in nums[1..].iter() {
            if s < 0 { s = i} else { s += i};
            max = std::cmp::max(max, s);
        }
      }
 max
}

// Option 2
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {

        let step = |cur: &mut i32, &n: &i32| {
            *cur = if *cur < 0 { n } 
             else { *cur + n };
            Some(*cur)
        };
        nums.iter()
            .scan(std::i32::MIN, step)
            .max()
            .unwrap()

    }
}

