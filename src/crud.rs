use std::collections::HashMap;
use std::io::{self, Write};

pub struct Client {
    alias:      String,
    name:       String,
    f_lastname: String,
    s_lastname: String,
    business:   String,
    rfc:        String,
    phone:      i32,
    email:      String
}

fn pause(){
    print!("Presiona enter para continuar ...  ");
    let _ = io::stdout().flush();
    let mut data = String::new();
    io::stdin().read_line(&mut data).unwrap();// get user input
}

fn get_field(field:&str)-> String{
    println!("{}",field);
    print!(">>> ");
    let _ = io::stdout().flush();
    let mut data = String::new();
    io::stdin().read_line(&mut data).unwrap();// get user input
    println!("");
    data.trim().to_string()
}

fn create_client()-> Client{
    let alias       = get_field("Alias");
    let name        = get_field("Nombre");
    let f_lastname  = get_field("Primer Apellido");
    let s_lastname  = get_field("Segundo Apellido");
    let business    = get_field("Razon Social");
    let rfc         = get_field("RFC");
    let phone       = get_field("Telefono").trim().parse::<i32>().unwrap();
    let email       = get_field("Correo");

    // return client
    Client{ alias, name, f_lastname, s_lastname, business, rfc, phone, email }
}

pub fn insert_client(clients:&mut HashMap<String,Client>){
    std::process::Command::new("clear").status().unwrap();// clear screen
    println!("-------------------------------\n\
              |     Creacion del Cliente    |\n\
              -------------------------------\n");
    let client = create_client();
    let alias  = client.alias[..].to_string(); //create an alias copy
    clients.insert(alias, client);//             insert in HashMap
    pause();
}

pub fn update_client(clients:&mut HashMap<String,Client>){
}

pub fn delete_client(clients:&mut HashMap<String,Client>){
    std::process::Command::new("clear").status().unwrap();// clear screen
    let alias = get_field("Alias");
    clients.remove(&alias);
}

pub fn show_clients(clients:&mut HashMap<String,Client>){
    std::process::Command::new("clear").status().unwrap();// clear screen
    println!("-------------------------------\n\
              |     Clientes Registrados    |\n\
              -------------------------------\n\
              |      alias -> nombre        |\n\
              -------------------------------");
    for (key, value) in clients.into_iter() {
        println!("  {:>10} -> {} {}", key, value.name, value.f_lastname);
    }
    pause();
}
