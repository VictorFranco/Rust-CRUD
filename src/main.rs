use std::io::{self, Write};
mod crud;

fn main() {
    std::process::Command::new("clear").status().unwrap();// clear screen
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
    io::stdin().read_line(&mut option).unwrap();//        get user input
    println!("");
    let option = option.trim().parse::<i32>().unwrap();// cast option
    match option {
        1 => crud::create_client(),
        2 => crud::update_client(),
        3 => crud::delete_client(),
        4 => crud::show_clients(),
        _ => println!("Esa no es una opcion valida")
    }
}
