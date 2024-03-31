use std::collections::HashMap;

use collections::{
    add_only_key_not_exists, concat_str_with_world, concat_string_with_world, count_words,
    list_hash_mpas, list_vector, multiple_vec_original,
};

fn main() {
    //VECTORS
    let mut vector = vec![1, 2, 3, 4];

    list_vector(&vector);
    multiple_vec_original(2, &mut vector);
    list_vector(&vector);

    //STRINGS
    let test_str = "Test";

    let test_string = String::from("test");
    //let test_string = "test".to_string();

    let res_str = concat_str_with_world(test_str);
    println!("STR:: {}", res_str);

    let res_string = concat_string_with_world(test_string.clone());
    println!("String:: {}", res_string);

    // Convert String -> &str
    let convert_in_str: &str = &test_string;
    println!("String convert in str: {}", convert_in_str);

    // Convert &str -> String
    let convert_in_str: &str = &test_str.to_string();
    println!("String convert in str: {}", convert_in_str);

    //Hash Maps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    list_hash_mpas(&scores);

    //Add value se nao existir
    let linha = format!("{}{}{}", "=".repeat(5), "ADD IF NOT EXIST", "=".repeat(5));
    println!("{}", linha);
    add_only_key_not_exists(&mut scores, "Red".to_string(), 5);
    add_only_key_not_exists(&mut scores, "Red".to_string(), 100);
    list_hash_mpas(&scores);

    //Update value of value prev
    let linha_count_words = format!("{}{}{}", "=".repeat(5), "COUNT WORDS", "=".repeat(5));
    println!("{}", linha_count_words);
    let text = String::from("hello world wonderful world");

    let map_text = count_words(text);
    list_hash_mpas(&map_text);
}
