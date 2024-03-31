use collections::{list_hash_mpas, list_string, list_vector, multiple_vec_original};

fn main() {
    //VECTORS
    let mut vector = vec![1, 2, 3, 4];

    list_vector(&vector);
    multiple_vec_original(2, &mut vector);
    list_vector(&vector);

    //STRINGS
}
