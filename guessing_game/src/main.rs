use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Adivinhe o numero!");

    let secret_number = rand::thread_rng().gen_range(1..=100);


    println!("O número é: {secret_number}");

    loop {

        println!("Entre com seu palpite.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("Seu palpite foi: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Chutou muito baixo"),
            Ordering::Greater => println!("Chutou muito alto"),
            Ordering::Equal => {
                println!("Você acertou!!");
                break;
            }
        }
    }

}
