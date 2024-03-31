use collections::{
    concat_str_with_world, concat_string_with_world, list_hash_mpas, list_vector,
    multiple_vec_original,
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
    list_hash_mpas();
}
