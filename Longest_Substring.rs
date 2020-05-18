use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn all_unique(arr: &Vec<char>, i: usize, j: usize) -> bool {
        let mut hash_set: HashSet<char> = HashSet::new();
        for k in i..(j + 1) {
            if hash_set.contains(&arr[k]) {
                return false;
            }
            hash_set.insert(arr[k]);
        }
        true
    }


    pub fn length_of_longest_substring(s: String) -> i32 {
               let mut hashMap: HashMap<&char, usize> = HashMap::new();
        let text: Vec<char> = s.chars().collect();
        let mut max = 0;
        let mut last_index = 0;
        for c in 0..text.len() {
            if hashMap.contains_key(&text[c]) {
                last_index = if hashMap.get(&text[c]).unwrap() + 1 > last_index {
                    hashMap.get(&text[c]).unwrap() + 1
                } else {
                    last_index
                };
            }
            max = cmp::max(c - last_index + 1, max);
            hashMap.insert(&text[c], c);
        }
        max as i32 
    }
}
