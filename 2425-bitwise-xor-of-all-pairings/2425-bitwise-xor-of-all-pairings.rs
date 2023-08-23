impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        if nums1.len() % 2 == 0 {
            if nums2.len() % 2 == 0 {
                0
            } else {
                nums1.iter().fold(0, |acc, &x| acc ^ x)
            }
        } else {
            if nums2.len() % 2 == 0 {
                nums2.iter().fold(0, |acc, &x| acc ^ x)
            } else {
                nums1.iter().fold(0, |acc, &x| acc ^ x) ^ nums2.iter().fold(0, |acc, &x| acc ^ x)
            }
        }
    }
}