use std::cmp::Ordering::Equal;
use std::io::{self, Write};

struct QuantumNumber(i32, i32, i32, String);

fn main() {
    println!("CÁLCULO DE POSIBLES NÚMEROS CUÁNTICOS");
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
            Err(_) => (),
        }
    };

    let l = n - 1;

    let ml = match n.cmp(&0) {
        Equal => vec![0],
        _ => ((-1 * l)..=(1 * l)).collect(),
    };

    #[allow(unused_variables)]
    let s = vec!["1/2", "-1/2"];

    #[allow(unused_variables)]
    #[allow(unused_mut)]
    let mut quantum_numbers: Vec<QuantumNumber> = Vec::new();

    for element in ml.into_iter() {
        println!("{}", element);
    }
}
