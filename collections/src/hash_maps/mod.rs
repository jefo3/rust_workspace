pub mod hash_maps {
    use std::collections::HashMap;

    pub fn list_hash_mpas(hash_map: &HashMap<String, i32>) {
        for (key, value) in hash_map {
            println!("Key: {}, Value: {}", key, value);
        }
    }

    pub fn add_only_key_not_exists(hash_map: &mut HashMap<String, i32>, key: String, value: i32) {
        hash_map.entry(key).or_insert(value);
    }

    pub fn count_words(text: String) -> HashMap<String, i32> {
        let mut hash_map: HashMap<String, i32> = HashMap::new();

        for word in text.split_whitespace() {
            let count = hash_map.entry(word.to_string()).or_insert(0);
            *count += 1;
        }

        return hash_map;
    }
}
