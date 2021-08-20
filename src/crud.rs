use std::collections::HashMap;
use std::io::{self, Write};

pub struct Client {
    alias:      String,
    name:       String,
    f_lastname: String,
    s_lastname: Option<String>,
    business:   String,
    rfc:        String,
    phone:      i32,
    email:      String
}

pub fn clear(){
    std::process::Command::new("clear").status().unwrap();// clear screen
}

pub fn pause(){
    print!("Presiona enter para continuar ...  ");
    let _ = io::stdout().flush();
    let mut data = String::new();
    io::stdin().read_line(&mut data).unwrap();// get user input
}

pub fn get_field(field:&str)-> String{
    println!("{}",field);
    print!(">>> ");
    let _ = io::stdout().flush();
    let mut data = String::new();
    io::stdin().read_line(&mut data).unwrap();// get user input
    println!("");
    data.trim().to_string()
}

fn create_client(alias:Option<String>)-> Client{
    let alias = match alias {
        Some(t) => t,//      if alias exists : use that value
        None    => get_field("Alias")// else : request the value
    };
    let name        = get_field("Nombre");
    let f_lastname  = get_field("Primer Apellido");
    let s_lastname  = get_field("Segundo Apellido (Opcional): Si no tienes presiona enter");
    let s_lastname  = match &s_lastname[..] {
        "" => None,//   if user doesn't have it : None
        t  => Some(t.to_string())//        else : Some(value)
    };
    let business    = get_field("Razon Social");
    let rfc         = get_field("RFC");
    let phone       = get_field("Telefono (Solo numeros)").trim().parse::<i32>().unwrap();
    let email       = get_field("Correo");

    // return client
    Client{ alias, name, f_lastname, s_lastname, business, rfc, phone, email }
}

pub fn insert_client(clients:&mut HashMap<String,Client>){
    clear();
    println!("-------------------------------\n\
              |     Creacion del Cliente    |\n\
              -------------------------------\n");
    let client = create_client(None);
    let alias  = client.alias[..].to_string(); //create an alias copy
    clients.insert(alias, client);//             insert in HashMap
    pause();
}

pub fn update_client(clients:&mut HashMap<String,Client>){
    clear();
    println!("-------------------------------\n\
              |     Modificar el Cliente    |\n\
              -------------------------------\n");
    let alias = get_field("Alias");
    match clients.get(&alias) {
        Some(_) => {},
        None    => panic!("Alias no encontrado"),
    }
    let client = create_client(Some(alias));
    let alias  = client.alias[..].to_string(); //create an alias copy
    clients.insert(alias, client);//             insert in HashMap
    pause();
}

pub fn delete_client(clients:&mut HashMap<String,Client>){
    clear();
    println!("-------------------------------\n\
              |     Eliminar el Cliente     |\n\
              -------------------------------\n");
    let alias = get_field("Alias");
    clients.remove(&alias);
}

pub fn show_clients(clients:&mut HashMap<String,Client>){
    clear();
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
