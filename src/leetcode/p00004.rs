struct Solution {}
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            return Solution::find_median_sorted_arrays(nums2, nums1);
        }
        let part_size = (nums1.len() + nums2.len() + 1) / 2;
        let mut l = 0;
        let mut r = nums1.len();
        while l <= r {
            let part1 = l + (r - l) / 2;
            let part2 = part_size - part1;
            let p1_l = if part1 == 0 {
                i32::MIN
            } else {
                nums1[part1 - 1]
            };
            let p2_l = if part2 == 0 {
                i32::MIN
            } else {
                nums2[part2 - 1]
            };
            let p1_r = if part1 == nums1.len() {
                i32::MAX
            } else {
                nums1[part1]
            };
            let p2_r = if part2 == nums2.len() {
                i32::MAX
            } else {
                nums2[part2]
            };
            let max_l = p1_l.max(p2_l);
            let min_r = p1_r.min(p2_r);
            if max_l <= min_r {
                return if (nums1.len() + nums2.len()) % 2 == 0 {
                    (max_l + min_r) as f64 / 2.0
                } else {
                    max_l as f64
                };
            }
            if p1_l > p2_r {
                r = part1 - 1;
            } else {
                l = part1 + 1;
            }
        }
        return 0.0;
    }
}

#[test]
fn test1() {
    let nums1 = vec![1, 2, 3, 4, 5];
    let nums2 = vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
    assert_eq!(9.0, Solution::find_median_sorted_arrays(nums1, nums2));
}

#[test]
fn test2() {
    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    assert_eq!(2.5, Solution::find_median_sorted_arrays(nums1, nums2));
}

#[test]
fn test3() {
    let nums1 = vec![];
    let nums2 = vec![1];
    assert_eq!(1.0, Solution::find_median_sorted_arrays(nums1, nums2));
}
