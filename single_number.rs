impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {  
      //using XOR operation
        let mut res = nums[0];
        for j in nums.iter(){
            res = res ^ nums[j]; 
    
        return res 
    }
}

