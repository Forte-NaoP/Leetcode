impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        use std::collections::HashMap;

        let mut coins = coins;
        coins.sort();

        let mut dp: HashMap<i32, HashMap<i32, i32>> = HashMap::new();

        dp.entry(0).or_insert(HashMap::new());
        for i in coins.iter() {
            dp.entry(0).and_modify(|e| {
                e.entry(*i).or_insert(0);
            });
        }

        for i in 1..=amount {
            for coin in coins.iter() {
                *dp.entry(i).or_insert(HashMap::new())
                    .entry(*coin).or_insert(0) = 
                    if i >= *coin {
                        if let Some(before) = dp.get(&(i-*coin)) {
                            let min = before.values().min().cloned().unwrap_or(i32::MAX);
                            if min == i32::MAX {
                                i32::MAX
                            } else {
                                min + 1
                            }
                        } else {
                            1
                        }
                    } else {
                        i32::MAX
                    }

            }
        }

        let ans = dp.get(&amount).unwrap().values().min().cloned().unwrap_or(i32::MAX);
        if ans == i32::MAX {
            -1
        } else {
            ans
        }    
    }
}