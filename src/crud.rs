use std::collections::HashMap;

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

pub fn create_client(clients:&mut HashMap<String,Client>){
}

pub fn update_client(clients:&mut HashMap<String,Client>){
}

pub fn delete_client(clients:&mut HashMap<String,Client>){
}

pub fn show_clients(clients:&mut HashMap<String,Client>){
}
