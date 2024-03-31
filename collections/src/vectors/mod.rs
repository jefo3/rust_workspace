pub mod vectors {
    pub fn list_vector(vector: &Vec<i32>) {
        //uso vector assim, pq aqui ja é a referencia do vec
        for element in vector {
            println!("Vector:: {}", element);
        }
    }

    pub fn multiple_vec_original(factor: i32, vector: &mut Vec<i32>) {
        for element in vector {
            *element *= factor;
        }
    }
}
