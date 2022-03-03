pub struct Solution;
impl Solution {
    fn vec_sum(arr: &Vec<i32>, val: &i32) -> i32 {
        // replaces the values of arr with val if the values are smaller
        let mut new_arr: Vec<i32> = Vec::new();
        for i in arr {
            if i > val {
                // new_arr.push(i32::from(&val + 0));
                new_arr.push(*val)
            } else {
                new_arr.push(*i);
            }
        }
        new_arr.iter().sum()
    }
    fn main_loop(arr: Vec<i32>, target: i32, n: i32) -> i32 {
        let next = &n + 1;
        let sum = Self::vec_sum(&arr, &n);
        let next_sum = Self::vec_sum(&arr, &next);
        let sum_difference = (&target - &sum).abs();
        let next_sum_difference = (&target - &next_sum).abs();
        // Base Case
        if sum == target {
            return n;
        } else if next_sum == target {
            return n + 1;
        } else if next_sum > target {
            if sum_difference <= next_sum_difference {
                return n;
            } else {
                return next;
            }
        } else if sum_difference == next_sum_difference {
            return n;
        } else {
            // Recursive Case
            return Self::main_loop(arr, target, next);
        }
    }
    /// Returns the sum of a mutated array closest to the given target
    ///
    /// # Arguments
    /// * `arr` - A vector that holds any number of integers
    /// * `target` - An integer target
    ///
    /// # Examples
    /// ```
    /// Structure::find_best_value(!vec[4, 9, 3], 3); // returns 3
    /// ```
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        Self::main_loop(arr, target, 0)
    }
}
