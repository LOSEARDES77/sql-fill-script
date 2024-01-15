use std::io::Write;
use rand::Rng;
use rand::seq::SliceRandom;
use std::fs;
use std::io;
use std::path;

#[derive(PartialEq)]
enum Campos{
    Nombre,
    Telefono,
    DNI,
    Fecha,
    Numero,
    Apellido,
    Mascota,
    Matricula,
}

fn main() {
    let mut ruta = String::new();
    print!("Introduce la ruta donde estan tus archivos sql: ");
    io::stdout().flush().expect("Print failed");
    io::stdin().read_line(&mut ruta).unwrap();
    let mut ruta = ruta.trim();

    if ruta == "." { ruta = "./" }

    if !path::Path::new(&ruta).exists() {
        println!("The directory does not exist.");
        std::process::exit(1);
    }


    let mut sqlfiles = Vec::new();
    let paths = fs::read_dir(&ruta).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap();
        if path.ends_with(".sql") || path.ends_with(".ddl") {
            sqlfiles.push(path.to_owned());
        }
    }

    for (i, file) in sqlfiles.iter().enumerate() {
        println!("{} - {}", i + 1, file);
    }

    let mut archivo = String::new();
    print!("Introduce el numero del archivo que deseas añadir datos: ");
    io::stdout().flush().expect("Print failed");
    io::stdin().read_line(&mut archivo).unwrap();

    let archivo = archivo.trim().parse::<usize>().unwrap();
    if archivo < 1 || archivo > sqlfiles.len() {
        println!("El numero introducido no es valido");
        std::process::exit(1);
    }

    let archivo = sqlfiles[archivo - 1].clone();
    println!("Archivo seleccionado: {}", archivo);

    let mut tabla = String::new();
    print!("Introduce el nombre de la tabla: ");
    io::stdout().flush().expect("Print failed");
    io::stdin().read_line(&mut tabla).unwrap();

    let mut necesidades = String::new();
    print!("Introduce que datos necesitas introducir (ej: \"nombre, dni, fecha, numero, telefono, apellido, mascota, matricula\"): ");
    io::stdout().flush().expect("Print failed");
    io::stdin().read_line(&mut necesidades).unwrap();



    let mut cantidad = String::new();
    print!("Cuantos inserts desead poner: ");
    io::stdout().flush().expect("Print failed");
    io::stdin().read_line(&mut cantidad).unwrap();

    let cantidad = cantidad.trim().parse::<usize>().unwrap();

    let necesidades: Vec<&str> = necesidades.split(", ").collect();
    let campos: Vec<Campos> = get_campos(&necesidades);

    let mut inserts = Vec::new();

    let mut contador = 0;
    while inserts.len() < cantidad {
        let insert = format!("\nINSERT INTO {} VALUES ({})", tabla.trim(), get_random_string(&campos));
        if !inserts.contains(&insert) {
            inserts.push(insert);
        }
        contador = contador + 1;
    }

    let mut f = fs::OpenOptions::new().append(true).open(ruta.to_owned() + "/" + &*archivo).unwrap();
    for insert in inserts {
        f.write_all(insert.as_bytes()).unwrap();
    }
    f.flush().unwrap();
    }

fn get_campos(necesidades: &Vec<&str>) -> Vec<Campos> {
    let mut campos = Vec::new();
    for i in necesidades {
        match i.trim() {
            "nombre" | "nom" => { campos.push(Campos::Nombre)}
            "telefono" | "tlf" => { campos.push(Campos::Telefono)}
            "dni" => { campos.push(Campos::DNI)}
            "fecha" | "f" => { campos.push(Campos::Fecha)}
            "numero" | "num"=> { campos.push(Campos::Numero)}
            "apellido" | "ape" => { campos.push(Campos::Apellido)}
            "mascota" | "mas" => { campos.push(Campos::Mascota)}
            "matricula" | "mat" => { campos.push(Campos::Matricula)}
            _ => {println!("El campo {} no es valido", i); std::process::exit(1)}
        }
    }
    return campos;
}

fn get_random_string(necesidades: &Vec<Campos>) -> String {
    let nombres = vec!["Pepe", "Juan", "Maria", "Luis", "Ana", "Pablo", "Sara", "Sandra", "Jorge", "Javier", "Raul", "Rosa", "Rocio", "Rafael", "Ramon", "Roberto", "Ricardo", "Rau", "Concepcion", "Carmen", "Cristina", "Cristian", "Cristobal", "Alberto", "Alvaro", "Alejandro", "Alba", "Adrian", "Adriana", "Alicia", "Ainhoa", "Aitor", "Aida", "Aitana", "Aina", "Ainara", "Aimar", "Aina", "Ainara", "Aimar", "Aitor", "Aitana", "Aida", "Adriana", "Adrian", "Alba", "Alejandro", "Alvaro", "Alberto", "Cristobal", "Cristian", "Cristina", "Carmen", "Concepcion", "Rau", "Ricardo", "Roberto", "Ramon", "Rafael", "Rocio", "Rosa", "Raul", "Javier", "Jorge", "Sandra", "Sara", "Pablo", "Ana", "Luis", "Maria", "Juan", "Pepe"];
    let apellidos = vec!["Garcia", "Rodriguez", "Gonzalez", "Fernandez", "Lopez", "Martinez", "Sanchez", "Perez", "Gomez", "Martin", "Jimenez", "Ruiz", "Hernandez", "Diaz", "Moreno", "Muñoz", "Alvarez", "Romero", "Alonso", "Gutierrez", "Navarro", "Torres", "Dominguez", "Vazquez", "Ramos", "Gil", "Ramirez", "Serrano", "Blanco", "Molina", "Morales", "Suarez", "Ortega", "Delgado", "Castro", "Ortiz", "Rubio", "Marin", "Sanz", "Nuñez", "Iglesias", "Medina", "Garrido", "Cortes", "Castillo", "Santos", "Lozano", "Guerrero", "Cano", "Prieto", "Mendez", "Cruz", "Calvo", "Gallego", "Herrera", "Marquez", "Leon", "Peña", "Flores", "Cabrera", "Campos", "Vega", "Fuentes", "Carrasco", "Diez", "Caballero", "Reyes", "Nieto", "Aguilar", "Pascual", "Santana", "Herrero", "Montero", "Hidalgo", "Lorenzo", "Gimenez", "Ibañez", "Ferrer", "Duran", "Santiago", "Benitez", "Vidal", "Vargas", "Carmona", "Crespo", "Pastor", "Roman", "Soto", "Saez", "Vicente", "Mora", "Soler", "Esteban", "Parra", "Bravo", "Gallardo", "Rojas", "Merino", "Franco", "Osorio", "Pardo", "Rivas", "Bravo", "Galán", "Moya", "Diego", "Vila", "Varela", "Arias", "Rey", "Moya", "Mora", "Vila", "Varela", "Arias", "Gonzalez", "Fernandez", "Lopez", "Martinez", "Sanchez", "Perez", "Gomez", "Martin", "Jimenez", "Ruiz", "Hernandez", "Diaz", "Moreno", "Muñoz", "Alvarez", "Romero", "Alonso", "Gutierrez", "Navarro", "Torres", "Dominguez", "Vazquez", "Ramos", "Gil", "Ramirez", "Serrano", "Blanco", "Molina", "Morales", "Suarez", "Ortega", "Delgado", "Castro", "Ortiz", "Rubio", "Marin", "Sanz", "Nuñez", "Iglesias", "Medina", "Garrido", "Cortes", "Castillo", "Santos", "Lozano", "Guerrero", "Cano", "Prieto", "Mendez", "Cruz", "Calvo", "Gallego", "Herrera", "Marquez", "Leon", "Peña", "Flores", "Cabrera", "Campos", "Vega", "Fuentes", "Carrasco", "Diez", "Caballero", "Reyes", "Nieto", "Aguilar", "Pascual", "Santana", "Herrero", "Montero", "Hidalgo", "Lorenzo", "Gimenez", "Ibañez", "Ferrer", "Duran", "Santiago", "Benitez", "Vidal", "Vargas", "Carmona", "Crespo", "Pastor", "Roman", "Soto", "Saez", "Vicente", "Mora", "Soler", "Esteban", "Parra", "Bravo", "Gallardo", "Rojas", "Merino", "Franco", "Osorio", "Pardo", "Rivas", "Bravo", "Galán", "Moya", "Diego", "Vila", "Varela", "Arias", "Rey", "Moya", "Mora", "Vila", "Varela", "Arias"];
    let mascotas = vec!["Chispas", "Pelusa", "Blanquito","Osito", "Luna", "Sol", "Lola", "Toby", "Rocky", "Max", "Lucy", "Sammy", "Daisy", "Coco", "Milo","Charlie", "Bailey", "Cooper", "Duke", "Bear", "Bella", "Ginger", "Zoey", "Sophie", "Molly", "Maggie", "Ruby", "Riley", "Emma" ];
    let mut res = String::new();
    let mut rng = rand::thread_rng();
    for i in necesidades {
        if i != &Campos::Numero && i != &Campos::Telefono {
            res += "\'";
        }
        match i {
            Campos::Apellido => {
                res += apellidos.choose(&mut rng).unwrap();
                res += " ";
                res += apellidos.choose(&mut rng).unwrap();
            }
            Campos::Nombre => { res += nombres.choose(&mut rng).unwrap() }
            Campos::Mascota => { res += mascotas.choose(&mut rng).unwrap() }
            Campos::Telefono => { res += &gen_telefono() }
            Campos::DNI => { res += &gen_dni() }
            Campos::Fecha => { res += &gen_fecha() }
            Campos::Numero => { res += &rng.gen_range(0..99999999).to_string()}
            Campos::Matricula => { res += &gen_matricula() }
        }
        if i != &Campos::Numero && i != &Campos::Telefono {
            res += "\'";
        }
        if i != &necesidades[necesidades.len() - 1] {
            res += ", ";
        }
    }

    return res;
}
fn gen_matricula() -> String {
    let mut rng = rand::thread_rng();
    let mut res = String::new();
    for _ in 0..4 {
        res.push(rng.gen_range(0..9).to_string().chars().next().unwrap());
    }
    res.push(' ');
    for _ in 0..3 {
        res.push(rng.gen_range('A'..='Z').to_string().chars().next().unwrap());
    }
    res
}

fn gen_dni() -> String {
    let mut rng = rand::thread_rng();
    let mut res = String::new();
    for _ in 0..8 {
        res += &rng.gen_range(0..9).to_string();
    }
    let position_string = "TRWAGMYFPDXBNJZSQVHLCKE";
    let position = res.parse::<usize>().unwrap() % 23;
    res += &position_string.chars().nth(position).unwrap().to_string();
    return res;
}

fn gen_telefono() -> String {
    let mut rng = rand::thread_rng();
    let mut res = String::new();
    res += "6";
    for _ in 0..8 {
        res += &rng.gen_range(0..9).to_string();
    }
    return res;
}

fn gen_fecha() -> String {
    let mut rng = rand::thread_rng();
    let mut res = String::new();
     res += &(rng.gen_range(1..31).to_string() + "/" + &rng.gen_range(1..31).to_string() + "/" + &rng.gen_range(1500..2024).to_string());
    return res;
}



