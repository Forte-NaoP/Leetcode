impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let tree = SegTree::new(&prices);
        let mut profit = -1;

        for i in 0..prices.len() {
            profit = std::cmp::max(profit, tree.query(0, prices.len()-1, 1, i+1, prices.len()-1) - prices[i]);
        }
        if profit <= 0 {
            0
        } else {
            profit
        }
    }
}

struct SegTree {
        pub tree: Vec<i32>
    }

    impl SegTree {
        pub fn new(arr: &Vec<i32>) -> Self {
            let tree = vec![0; arr.len()*4];

            let mut segtree = SegTree { tree };
            segtree.init(arr, 0, arr.len()-1, 1);
            segtree
        }

        fn init(&mut self, arr: &Vec<i32>, st: usize, ed: usize, idx: usize) -> i32 {
            if st == ed {
                self.tree[idx] = arr[st];
                return self.tree[idx];
            }

            let mid = (st + ed) / 2;
            self.tree[idx] = std::cmp::max(self.init(arr, st, mid, idx*2), self.init(arr, mid+1, ed, idx*2+1));
            return self.tree[idx];
        }

        pub fn query(&self, st: usize, ed: usize, idx: usize, left: usize, right: usize) -> i32 {
            if st > right || ed < left {
                return -1
            }
            if right >= ed && st >= left {
                return self.tree[idx];
            }
            
            let mid = (st + ed) / 2;
            return std::cmp::max(self.query(st, mid, idx*2, left, right), self.query(mid+1, ed, idx*2+1, left, right)); 
        }
    }