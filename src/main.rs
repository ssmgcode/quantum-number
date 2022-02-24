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
    println!("CÁLCULO DE POSIBLES NÚMEROS CUÁNTICOS DE UN ELECTRÓN");
    println!();

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
        println!();
        println!("PROCEDIMIENTO");
        match entered_energy_level.trim().parse::<i32>() {
            Ok(number) => {
                println!("n = {}", number);
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
    println!("l = n - 1 = {} - 1 = {}", energy_level, azimuth_number);
    // -> l

    // <- ml
    let magnetic_numbers = match energy_level.cmp(&0) {
        Equal => vec![0],
        _ => ((-1 * azimuth_number)..=(1 * azimuth_number)).collect(),
    };
    println!("ml = [-l, 0, l] = {:?}", magnetic_numbers);
    // ->

    // <- s
    let spin_numbers = vec!["+1/2", "-1/2"];
    println!("s = {:?}", spin_numbers);
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
    println!("RESULTADOS");
    let mut does_have_to_change_spin_number = false;
    for quantum_number in quantum_numbers.into_iter() {
        match does_have_to_change_spin_number {
            false => {
                println!("> {}", quantum_number);
                does_have_to_change_spin_number = true;
            }
            true => {
                println!("  {}", quantum_number);
                println!();
                does_have_to_change_spin_number = false;
            }
        }
    }
}
