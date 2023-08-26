impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        let mut group: HashMap<String, Vec<String>>  = HashMap::new();

        for str in strs {
            let mut chars: Vec<char> = str.chars().collect();
            chars.sort();
            let sorted: String = chars.into_iter().collect();
            group.entry(sorted).or_insert(vec![]).push(str);
        }

        group.into_iter().map(|(_, value)| value).collect()
    }
}