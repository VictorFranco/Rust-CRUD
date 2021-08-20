use std::collections::HashMap;
use std::io::{self, Write};
use std::process;
mod crud;

fn main() {
    crud::clear();

    let security = crud::get_field("Palabra de seguridad");
    if security!=String::from("password") {
        println!("Palabra incorrecta");
        process::exit(1);
    }

    let mut clients:HashMap<String,crud::Client> = HashMap::new();

    loop {
        crud::clear();
        println!("-------------------------------\n\
                  |            Menu             |\n\
                  -------------------------------\n\
                  |                             |\n\
                  |    1. Agregar Cliente       |\n\
                  |    2. Modificar Cliente     |\n\
                  |    3. Eliminar Cliente      |\n\
                  |    4. Lista Clientes        |\n\
                  |    5. Salir                 |\n\
                  |                             |\n\
                  -------------------------------");
        let option = crud::get_field("");
        let option = option.trim().parse::<i32>().unwrap();// cast option
        match option {
            1 => crud::insert_client(&mut clients),
            2 => crud::update_client(&mut clients),
            3 => crud::delete_client(&mut clients),
            4 => crud::show_clients(&mut clients),
            5 => break,
            _ => {
                    println!("Esa no es una opcion valida");
                    crud::pause();
                 }
        }
    }
}
