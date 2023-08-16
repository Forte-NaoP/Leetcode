impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        use std::collections::{VecDeque, HashMap, HashSet};

        let mut reachable = 1;
        let mut q = VecDeque::new();
        let mut tree = HashMap::new();
        let restricted: HashSet<i32> = restricted.into_iter().collect();
        let mut visit = vec![false; n as usize];

        for edge in edges.iter() {
            tree.entry(edge[0]).or_insert(vec![]).push(edge[1]);
            tree.entry(edge[1]).or_insert(vec![]).push(edge[0]);
        }

        q.push_back(0);
        visit[0] = true;

        while !q.is_empty() {
            let node = q.pop_front().unwrap();

            for i in tree.get(&node).unwrap() {
                if !restricted.contains(i) && !visit[*i as usize] {
                    q.push_back(*i);
                    visit[*i as usize] = true;
                    reachable += 1;
                } 
            }
        }


        reachable
    }
}