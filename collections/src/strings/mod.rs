pub mod strings {
    pub fn concat_string_with_world(mut s: String) -> String {
        let world = String::from(", World");

        s.push_str(&world);
        s
    }

    pub fn concat_str_with_world(s: &str) -> String {
        let world = ", World";

        let concat = format!("{}{}", s, world);
        // let concat = s.to_string() + world;

        concat
    }
}
