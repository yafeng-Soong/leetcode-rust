use crate::solution::Solution;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let (m, n) = (nums1.len(), nums2.len());
        let mut heap = BinaryHeap::new();
        for i in 0..m {
            let tmp = Pair {
                a: nums1[i],
                b: nums2[0],
            };
            if heap.len() >= k as usize && tmp.cmp(&heap.peek().unwrap()) == Ordering::Greater {
                break;
            }

            for j in 0..n {
                let pair = Pair {
                    a: nums1[i],
                    b: nums2[j],
                };

                if heap.len() < k as usize {
                    heap.push(pair);
                    continue;
                }

                match pair.cmp(&heap.peek().unwrap()) {
                    Ordering::Less => {
                        heap.pop();
                        heap.push(pair);
                    }
                    _ => break,
                }
            }
        }

        heap.into_vec().iter().map(|x| vec![x.a, x.b]).collect()
    }
}

struct Pair {
    a: i32,
    b: i32,
}

impl Eq for Pair {}
impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        let sum = self.a + self.b;
        sum.cmp(&(other.a + other.b))
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
