impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut indegree = vec![0; num_courses as usize];

        use std::collections::HashMap;
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut sorted =vec![];
        let mut q = vec![];

        for v in prerequisites.iter() {
            let (depend, prereq) = (v[0] as usize, v[1] as usize);
            indegree[depend] += 1;
            graph.entry(prereq).or_insert(vec![]).push(depend);
        }

        for i in 0..indegree.len() {
            if indegree[i] == 0 {
                q.push(i);
            }
        }

        while !q.is_empty() {
            let c = q[0];
            q.swap_remove(0);
            sorted.push(c+1);
            if let Some(v) = graph.get(&c) {
                for d in v.iter() {
                    indegree[*d] -= 1;
                    if indegree[*d] == 0 {
                        q.push(*d);
                    }
                }
            } 

        }

        for i in 0..indegree.len() {
            if indegree[i] != 0 {
                return false
            }
        }

        true
    }

}