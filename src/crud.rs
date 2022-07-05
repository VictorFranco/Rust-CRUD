use std::collections::HashMap;
use std::io::{self, Write};

pub struct Client {
    alias:      String,
    name:       String,
    f_lastname: String,
    s_lastname: Option<String>,
    business:   String,
    itin:       String,
    phone:      i64,
    email:      String
}

pub fn clear(){
    std::process::Command::new("clear").status().unwrap();// clear screen
}

pub fn pause(){
    print!("Press ENTER to continue ...  ");
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
    let name        = get_field("Name");
    let f_lastname  = get_field("First Last Name");
    let s_lastname  = get_field("Second Last Name (Optional): if you don't have press ENTER");
    let s_lastname  = match &s_lastname[..] {
        "" => None,//   if user doesn't have it : None
        t  => Some(t.to_string())//        else : Some(value)
    };
    let business    = get_field("Business");
    let itin        = get_field("ITIN");
    let phone       = get_field("Phone (only numbers)").trim().parse::<i64>().unwrap();
    let email       = get_field("Email");

    // return client
    Client{ alias, name, f_lastname, s_lastname, business, itin, phone, email }
}

pub fn insert_client(clients:&mut HashMap<String,Client>){
    clear();
    println!("-------------------------------\n\
              |        Create Client        |\n\
              -------------------------------\n");
    let client = create_client(None);
    let alias  = client.alias[..].to_string(); //create an alias copy
    clients.insert(alias, client);//             insert in HashMap
    pause();
}

pub fn update_client(clients:&mut HashMap<String,Client>){
    clear();
    println!("-------------------------------\n\
              |        Update Client        |\n\
              -------------------------------\n");
    let alias = get_field("Alias");
    match clients.get(&alias) {
        Some(_) => {},
        None    => panic!("Alias not found"),
    }
    let client = create_client(Some(alias));
    let alias  = client.alias[..].to_string(); //create an alias copy
    clients.insert(alias, client);//             insert in HashMap
    pause();
}

pub fn delete_client(clients:&mut HashMap<String,Client>){
    clear();
    println!("-------------------------------\n\
              |        Delete Client        |\n\
              -------------------------------\n");
    let alias = get_field("Alias");
    clients.remove(&alias);
}

pub fn read_clients(clients:&mut HashMap<String,Client>){
    clear();
    println!("-------------------------------\n\
              |      Registred Clients      |\n\
              -------------------------------\n");
    for (_, value) in clients.into_iter() {
        let Client{ alias, name, f_lastname, s_lastname, business, itin, phone, email } = value;
        println!("Alias: {}\n\
                  Name: {} {} {}\n\
                  Business: {}\n\
                  ITIN: {}\n\
                  Phone: {}\n\
                  Email: {}\n", 
                  alias, 
                  name, 
                  f_lastname,
                  s_lastname.as_ref().unwrap_or(&String::from("")),
                  business,
                  itin,
                  phone,
                  email);
    }
    pause();
}
