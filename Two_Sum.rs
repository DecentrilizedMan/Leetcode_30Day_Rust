/* 
   given nums = [2,7,11,15] , target = 9
   return [0,1]  // on balf of its positions in the vector  
*/

use std::collections::HashMap;

impl Solution{
  pub fn two_sum(nums:Vec<i32>,target:i32) -> Vec<i32>{
     let mut seen = HashMap:: new();
     for(_i,_j) in nums.iter().enumerate() {
         if seen.contains_key(_j){
             return vec![seen[_j],_i as i32] ;
         }else {
                seen.insert(target-_j, _i as i32) ;
             }
            
      
         }
         vec![] 
     }
  } 


  fn main() {
    
}

