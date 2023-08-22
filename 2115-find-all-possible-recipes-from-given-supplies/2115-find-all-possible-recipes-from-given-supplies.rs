impl Solution {
    pub fn find_all_recipes(recipes: Vec<String>, ingredients: Vec<Vec<String>>, supplies: Vec<String>) -> Vec<String> {
        use std::collections::{HashMap, HashSet, VecDeque};

        let supplies: HashSet<&str> = supplies.iter().map(|s| s.as_str()).collect();
        let mut can_make: HashSet<&str> = HashSet::new();
        let mut build: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut indegree: HashMap<&str, i32> = HashMap::new();
        let mut q: VecDeque<&str> = VecDeque::new();

        for sup in supplies.iter() {
            indegree.entry(sup).or_insert(0);
        }

        for (idx, recipe) in recipes.iter().enumerate() {
            for ing in ingredients[idx].iter() {
                build.entry(ing.as_str()).or_insert(vec![]).push(recipe.as_str());
                *indegree.entry(recipe.as_str()).or_insert(0) += 1;
            }
        }

        for ing in indegree.iter() {
            if *ing.1 == 0 {
                q.push_back(ing.0);
            }
        }

        while let Some(ing) = q.pop_front() {
            can_make.insert(ing);
            if let Some(result) = build.get(&ing) {
                for recipe in result.iter() {
                    indegree.entry(recipe).and_modify(|e| {
                        *e -= 1;
                        if *e == 0 {
                            q.push_back(*recipe);
                        }
                    });
                }
            }
        }

        can_make.difference(&supplies).map(|r| r.to_string()).collect()

    }
}