use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.max_heap.push(num);
        if self.max_heap.len() > self.min_heap.len() + 1 {
            let x = self.max_heap.pop().unwrap();
            self.min_heap.push(Reverse(x));
        }

        if self.min_heap.len() > 0
            && self.max_heap.peek().unwrap() > &self.min_heap.peek().unwrap().0
        {
            let (a, b) = (self.max_heap.pop().unwrap(), self.min_heap.pop().unwrap().0);
            self.max_heap.push(b);
            self.min_heap.push(Reverse(a));
        }
    }

    fn find_median(&self) -> f64 {
        if self.max_heap.len() == self.min_heap.len() {
            let (a, b) = (
                self.max_heap.peek().unwrap(),
                self.min_heap.peek().unwrap().0,
            );
            return (a + b) as f64 / 2.0;
        }

        return *self.max_heap.peek().unwrap() as f64;
    }
}
