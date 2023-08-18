impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        use std::collections::HashMap;

        let mut ranks = HashMap::new();
        let mut sort = vec![];

        for (i, char) in order.chars().enumerate() {
            *ranks.entry(char).or_insert(0) = i;
        }

        for char in s.chars() {
            let rank = ranks.get(&char).or(Some(&27)).unwrap();
            sort.push(Order {val: char, rank: *rank});
        }

        sort.sort_by_key(|k| k.rank);

        let mut sorted = String::new();
        for char in sort.iter() {
            sorted.push(char.val);
        }

        sorted
    }
}

pub struct Order {
    pub val: char,
    pub rank: usize,
}