use std::io;

fn main() {
    println!("Advinhe o numero!");

    println!("Digite o seu palpite.");

    let mut palpite = String::new();

    io::stdin().read_line(&mut palpite)
        .expect("Falha ao ler entrada");

    println!("Voce disse: {}", palpite);
}
