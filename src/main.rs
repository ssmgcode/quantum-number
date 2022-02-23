use colored::Colorize;
use std::cmp::Ordering::Equal;
use std::fmt::{self, Display, Formatter};
use std::io::{self, Write};

struct QuantumNumber(i32, i32, i32, String);

impl Display for QuantumNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    println!("CÁLCULO DE POSIBLES NÚMEROS CUÁNTICOS");

    // <- Energy level (n)
    let energy_level = loop {
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
    let azimuth_number = energy_level - 1;
    // -> l

    // <- ml
    let magnetic_numbers = match energy_level.cmp(&0) {
        Equal => vec![0],
        _ => ((-1 * azimuth_number)..=(1 * azimuth_number)).collect(),
    };
    // ->

    // <- s
    let spin_numbers = vec!["1/2", "-1/2"];
    // ->

    let mut quantum_numbers: Vec<QuantumNumber> = Vec::new();
    for magnetic_number in magnetic_numbers.into_iter() {
        for spin_number in spin_numbers.iter() {
            quantum_numbers.push(QuantumNumber(
                energy_level,
                azimuth_number,
                magnetic_number,
                String::from(*spin_number),
            ))
        }
    }

    println!();
    let mut counter: u8 = 0;
    for quantum_number in quantum_numbers.into_iter() {
        match counter {
            0 => println!(">{}", quantum_number),
            1 => println!(" {}", quantum_number),
            _ => {
                println!();
                counter = 0;
            }
        }
    }
}
