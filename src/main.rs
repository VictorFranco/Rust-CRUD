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
}
