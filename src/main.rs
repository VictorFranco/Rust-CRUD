use std::collections::HashMap;
use std::process;
mod crud;

fn main() {
    crud::clear();

    let security = crud::get_field("Password");
    if security!=String::from("password") {
        println!("The password is incorrect");
        process::exit(1);
    }

    let mut clients:HashMap<String,crud::Client> = HashMap::new();

    loop {
        crud::clear();
        println!("-------------------------------\n\
                  |            Menu             |\n\
                  -------------------------------\n\
                  |                             |\n\
                  |      1. Create Client       |\n\
                  |      2. Read Clients        |\n\
                  |      3. Delete Client       |\n\
                  |      4. Update Client       |\n\
                  |      5. Exit                |\n\
                  |                             |\n\
                  -------------------------------");
        let option = crud::get_field("");
        let option = option.trim().parse::<i32>().unwrap();// cast option
        match option {
            1 => crud::insert_client(&mut clients),
            2 => crud::read_clients(&mut clients),
            3 => crud::update_client(&mut clients),
            4 => crud::delete_client(&mut clients),
            5 => break,
            _ => {
                    println!("Invalid option");
                    crud::pause();
                 }
        }
    }
}
