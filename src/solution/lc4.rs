use crate::solution::Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (m, n) = (nums1.len(), nums2.len());
        let need = (m + n) / 2;
        fn find(nums1: &[i32], nums2: &[i32], k: usize) -> f64 {
            let (mut nums1, mut nums2) = (nums1, nums2);
            if nums1.len() > nums2.len() {
                (nums1, nums2) = (nums2, nums1);
            }

            let (m, n) = (nums1.len(), nums2.len());
            if m == 0 {
                return nums2[k - 1] as f64;
            }

            if k == 1 {
                return nums1[0].min(nums2[0]) as f64;
            }

            let offset = k / 2;
            let (i, j) = (m.min(offset), n.min(offset));

            if nums1[i - 1] < nums2[j - 1] {
                find(&nums1[i..], nums2, k - i)
            } else {
                find(nums1, &nums2[j..], k - j)
            }
        }

        if (m + n) % 2 == 1 {
            find(&nums1, &nums2, need + 1)
        } else {
            (find(&nums1, &nums2, need) + find(&nums1, &nums2, need + 1)) / 2.0
        }
    }
}

#[test]
fn test() {
    struct Test {
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        expected: f64,
    }

    let tests = vec![
        Test {
            nums1: vec![1, 2, 6],
            nums2: vec![3, 4, 5, 7, 8, 9, 10],
            expected: 5.5,
        },
        Test {
            nums1: vec![1, 2, 2],
            nums2: vec![1, 2, 3],
            expected: 2.0,
        },
        Test {
            nums1: vec![1, 2],
            nums2: vec![],
            expected: 1.5,
        },
        Test {
            nums1: vec![1, 3],
            nums2: vec![2],
            expected: 2.0,
        },
        Test {
            nums1: vec![1, 2],
            nums2: vec![3, 4],
            expected: 2.5,
        },
        Test {
            nums1: vec![1, 2],
            nums2: vec![1, 2, 3, 4, 5, 6],
            expected: 2.5,
        },
        Test {
            nums1: vec![1, 2, 3, 4, 5, 6],
            nums2: vec![1, 2],
            expected: 2.5,
        },
        Test {
            nums1: vec![1, 2],
            nums2: vec![1, 2, 3, 4, 5, 6, 7],
            expected: 3.0,
        },
        Test {
            nums1: vec![0, 0, 0, 0, 0],
            nums2: vec![-1, 0, 0, 0, 0, 0, 1],
            expected: 0.0,
        },
        Test {
            nums1: vec![2, 2, 4, 4],
            nums2: vec![2, 2, 2, 4, 4],
            expected: 2.0,
        },
        Test {
            nums1: vec![17],
            nums2: vec![],
            expected: 17.0,
        },
        Test {
            nums1: vec![],
            nums2: vec![17],
            expected: 17.0,
        },
        Test {
            nums1: vec![1, 2, 3, 4, 5],
            nums2: vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
            expected: 9.0,
        },
        Test {
            nums1: vec![1, 2, 2],
            nums2: vec![1, 2, 3],
            expected: 2.0,
        },
    ];

    for t in tests {
        assert_eq!(
            Solution::find_median_sorted_arrays(t.nums1, t.nums2),
            t.expected
        );
    }
}
