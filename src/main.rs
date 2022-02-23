use colored::Colorize;
use std::cmp::Ordering::Equal;
use std::io::{self, Write};

struct QuantumNumber(i32, i32, i32, String);

fn main() {
    println!("CÁLCULO DE POSIBLES NÚMEROS CUÁNTICOS");

    // <- Energy level (n)
    let n = loop {
        print!("Nivel de energía: ");
        io::stdout().flush().unwrap();

        let mut entered_energy_level = String::new();
        // Ask for energy level
        io::stdin()
            .read_line(&mut entered_energy_level)
            .expect("Hubo un error al leer el número");

        // Convert entered energy level to a valid number
        match entered_energy_level.trim().parse::<i32>() {
            Ok(number) => {
                break number;
            }
            Err(_) => println!(
                "{}",
                "El número cuántico debe ser un número. Inténtalo de nuevo.\n".red()
            ),
        }
    };
    // ->

    // <- l
    let l = n - 1;
    // -> l

    // <- ml
    let ml = match n.cmp(&0) {
        Equal => vec![0],
        _ => ((-1 * l)..=(1 * l)).collect(),
    };
    // ->

    // <- s
    let s = vec!["1/2", "-1/2"];
    // ->

    let mut quantum_numbers: Vec<QuantumNumber> = Vec::new();

    for element in ml.into_iter() {
        println!("{}", element);
    }
}
