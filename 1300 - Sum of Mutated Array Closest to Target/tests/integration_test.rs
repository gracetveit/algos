use sum_of_mutated_array_closest_to_target::Solution;

#[test]
fn four_nine_three_target_ten() {
    assert_eq!(Solution::find_best_value(vec![4, 9, 3], 10), 3);
}

#[test]
fn two_three_five_target_ten() {
    assert_eq!(Solution::find_best_value(vec![2, 3, 5], 10), 5);
}

#[test]
fn lots_of_numbers() {
    assert_eq!(
        Solution::find_best_value(vec![60864, 25176, 27249, 21296, 20204], 56803),
        11361
    )
}
