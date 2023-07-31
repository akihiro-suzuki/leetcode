use std::cmp::max;

fn main() {
    println!("Hello, world!");
}
struct Solution;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let max_len = strs.iter().map(|s| s.len()).max().unwrap_or(0);
        let mut prefixes: Vec<char> = vec![];
        let chars_list = strs
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        'label: for i in 0..max_len {
            let mut prev = None;
            for s in chars_list.iter() {
                if let Some(c) = s.get(i) {
                    if prev.is_some() {
                        if prev != Some(c) {
                            break 'label;
                        }
                    }
                    prev = Some(c);
                } else {
                    break 'label;
                }
            }
            prefixes.push(prev.unwrap().clone());
        }
        prefixes.into_iter().collect::<String>()
    }
}
