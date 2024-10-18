use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("!Adivina el numero!");

    let numero_aleatorio = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Ingrese un valor.");

        let mut valor = String::new();

        io::stdin()
            .read_line(&mut valor)
            .expect("Fallo al leer el valor.");

        let valor: u32 = match valor.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Intenta de nuevo");
                continue;
            }
        };

        println!("Valor digitado: {}", valor);

        match valor.cmp(&numero_aleatorio) {
            Ordering::Less => println!("Numero por debajo"),
            Ordering::Greater => println!("Numero por encima"),
            Ordering::Equal => {
                println!("¡Acertaste!");
                break;
            }
        }
    }
}
