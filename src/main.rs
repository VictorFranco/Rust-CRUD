use std::io::{self, Write};

fn main() {
    std::process::Command::new("clear").status().unwrap();
    println!("-------------------------------\n\
              |            Menu             |\n\
              -------------------------------\n\
              |                             |\n\
              |    1. Agregar Cliente       |\n\
              |    2. Modificar Cliente     |\n\
              |    3. Eliminar Cliente      |\n\
              |    4. Lista Clientes        |\n\
              |                             |\n\
              -------------------------------\n");
    print!(">>> ");
    let _ = io::stdout().flush();
    let mut option = String::new();
    io::stdin().read_line(&mut option).unwrap();
    let option = option.trim().parse::<i32>().unwrap();
    println!("{}",option);
}
