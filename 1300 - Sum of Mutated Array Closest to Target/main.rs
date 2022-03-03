// When given an array and a target, find the value that when every value in the
// given array larger than the value is changed, is summed to be the closest (in
// absolute difference) to the target

struct Solution {}

impl Solution {
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
      // The *value cannot be higher than the largest value in the given array.

      // Start with 0, and keep track of the sum.

      // If the next sum is closer to the target, get rid of the old sum

      // If the sum is greater than target, and the sum is farther away than the
      // previous sum, return previous sum.
      3
    }
}

fn main() {
  println!("{} == 3?", Solution::find_best_value(vec![4,9,3], 10));
  println!("{} == 5?", Solution::find_best_value(vec![2, 3, 5], 10));
  println!("{} == 11361?", Solution::find_best_value(vec![60864,25176,27249,21296,20204], 56803));
}
