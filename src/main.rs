use std::io;

fn main() {
    println!("Adivina el numero!");
    println!("Ingrese un valor.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Fallo al leer la linea");

    println!("Valor digitado: {}", guess);
}
