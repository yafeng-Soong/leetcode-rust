mod lc155 {
    struct MinStack {
        data: Vec<i32>,
        cur_min: i32,
    }

    /**
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    /**
     * Your MinStack object will be instantiated and called as such:
     * let obj = MinStack::new();
     * obj.push(val);
     * obj.pop();
     * let ret_3: i32 = obj.top();
     * let ret_4: i32 = obj.get_min();
     */
    impl MinStack {
        fn new() -> Self {
            MinStack {
                data: Vec::new(),
                cur_min: i32::MAX,
            }
        }

        fn push(&mut self, val: i32) {
            self.cur_min = self.cur_min.min(val);
            self.data.push(val);
        }

        fn pop(&mut self) {
            let top = self.data.pop().unwrap();
            if top == self.cur_min {
                self.cur_min = i32::MAX;
                for i in 0..self.data.len() {
                    self.cur_min = self.cur_min.min(self.data[i]);
                }
            }
        }

        fn top(&self) -> i32 {
            self.data[self.data.len() - 1]
        }

        fn get_min(&self) -> i32 {
            self.cur_min
        }
    }
}
