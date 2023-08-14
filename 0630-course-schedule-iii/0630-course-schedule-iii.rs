impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        let mut courses = courses;
        let mut pq: BinaryHeap<i32> = BinaryHeap::new();
        let mut day = 0;

        courses.sort_by_key(|v| {
            v[1]
        });

        for course in courses.iter() {
            let limit = course[1];
            if day + course[0] <= limit {
                pq.push(course[0]);
                day += course[0];
            } else if !pq.is_empty() {
                let last_max = pq.peek().unwrap();
                if day + course[0] - last_max < limit && course[0] < *last_max {
                    day -= last_max;
                    pq.pop().unwrap();
                    pq.push(course[0]);
                    day += course[0];
                }
            }

        }

        pq.len() as i32
    }
}
