use std::io::Write;
use rand::Rng;
use rand::seq::SliceRandom;
use std::fs;
use std::io;
use std::path;

enum Campos{
    Nombre,
    Telefono,
    DNI,
    Fecha,
    Numero
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
    print!("Introduce que datos necesitas introducir (ej: \"nombre, dni, fecha, numero, telefono\"): ");
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
            _ => {println!("El campo {} no es valido", i); std::process::exit(1)}
        }
    }
    return campos;
}

fn get_random_string(necesidades: &Vec<Campos>) -> String {
    let nombres = vec!["Pepe", "Juan", "Maria", "Luis", "Ana", "Pablo", "Sara", "Sandra", "Jorge", "Javier", "Raul", "Rosa", "Rocio", "Rafael", "Ramon", "Roberto", "Ricardo", "Rau", "Concepcion", "Carmen", "Cristina", "Cristian", "Cristobal", "Alberto", "Alvaro", "Alejandro", "Alba", "Adrian", "Adriana", "Alicia", "Ainhoa", "Aitor", "Aida", "Aitana", "Aina", "Ainara", "Aimar", "Aina", "Ainara", "Aimar", "Aitor", "Aitana", "Aida", "Adriana", "Adrian", "Alba", "Alejandro", "Alvaro", "Alberto", "Cristobal", "Cristian", "Cristina", "Carmen", "Concepcion", "Rau", "Ricardo", "Roberto", "Ramon", "Rafael", "Rocio", "Rosa", "Raul", "Javier", "Jorge", "Sandra", "Sara", "Pablo", "Ana", "Luis", "Maria", "Juan", "Pepe" ];
    let dnis = vec![ "98753944L", "18373972E", "24652592G", "54478417T", "62975314J", "95822676K", "67255688U", "92284716H", "85629726J", "32766789W", "48592247Q", "52554824U", "71783497N", "12769372R", "64824859O", "19363191G", "84357567B", "69111848P", "92571871F", "46997723K", "98448558P", "76771944D", "49964121G", "59917886L", "36991183I", "69513883C", "47152864M", "57975297A", "48938578M", "59449971A", "49618991C", "38891127W", "53183952F", "62767886M", "24927243G", "35153988S", "31632687J", "16922879U", "21677764R", "63553681B", "25185446L", "47273611D", "39551732C", "43387245F", "83179161I", "92537351V", "15537416O", "11927499C", "61653796H", "53963728N", "23626234O", "36248399A", "71719243F", "99728633Q", "57173798E", "99256853G", "57177913J", "27297586U", "99455173C", "93853872D", "79894713R", "43841574F", "43668783H", "86317812B", "57596486V", "17188353G", "22461582L", "83238325G", "73132271P", "14523338M", "35484648V", "17261186W", "72751925E", "85339691H", "34389311P", "16886336C", "19385353B", "59717431V", "46689456L", "86298525C", "21796845A", "24312821L", "81647546B", "66921615D", "91395159K", "93224489R", "48917678F", "37321761Q", "29367947T", "57116243F", "18272721K", "84455683F", "53537721W", "67471392B", "92663363Q", "22415768L", "47175644J", "31914577H", "36333991P", "69655835M"];
    let telefonos = vec!["611998197", "617618388", "670945223", "649692657", "661025928", "684120967", "668971924", "694562052", "625125002", "654447117", "679832756", "657007694", "668678737", "614009503", "652390321", "657367636", "683889509", "685201986", "663110060", "623349894", "671793622", "663050846", "616422101", "693234052", "656866719", "669476263", "632371019", "627450496", "621828009", "637802547", "674672716", "661099195", "682221289", "629204483", "698057875", "634778852", "613560492", "618304018", "694570547", "648349079", "612463826", "616708737", "654286241", "699681235", "682677927", "643531028", "644112554", "648116082", "640393537", "632190454", "615324277", "661363154", "632336441", "656880848", "685980229", "661433777", "645314311", "660294207", "638062454", "691225803", "628921701", "642022184", "697241460", "617631896", "618374150", "689729346", "636187583", "610537799", "641092678", "654794645", "683001363", "635718107", "673774139", "616393098", "629251961", "611413377", "686612283", "610975981", "629438779", "638929867", "636392305", "692349856", "699020927", "692680096", "620025557", "691652320", "655081436", "628549212", "649875537", "618739053", "633322783", "632578098", "687153362", "676152264", "695828839", "657496357", "646410417", "631620009", "650621464", "653766417"];
    let fechas = vec!["6/6/1956", "5/9/1964", "14/1/1997", "5/12/1976", "24/2/1997", "5/3/1972", "10/8/1999", "7/6/1966", "21/3/1954", "8/6/1978", "27/6/1964", "24/8/1959", "7/6/1990", "11/12/1979", "3/11/1953", "25/12/1960", "2/6/1989", "23/7/2001", "13/6/1989", "8/1/1952", "3/8/1954", "4/3/1985", "17/10/1966", "19/6/1995", "2/8/1963", "6/6/1957", "12/4/2001", "21/3/1999", "15/8/1971", "19/9/1966", "7/10/2000", "4/9/1983", "16/7/1989", "9/12/1958", "13/9/1950", "18/5/1960", "3/10/2000", "15/11/1998", "10/11/1982", "24/8/1958", "17/10/1976", "9/4/1993", "10/1/1957", "28/9/1989", "7/11/1998", "8/1/1990", "20/7/1982", "6/11/1971", "6/9/1984", "8/5/1965", "7/12/1953", "28/6/1950", "14/10/1976", "23/4/1987", "22/8/1983", "7/5/1986", "12/2/1991", "13/2/1955", "2/4/1964", "14/9/1995", "26/6/1991", "12/11/1970", "3/11/2000", "7/5/1970", "27/4/1994", "17/11/1985", "14/11/1961", "17/5/1990", "26/3/1963", "24/12/1964", "22/6/1968", "21/8/1974", "8/3/1999", "22/10/1972", "8/7/1956", "7/9/1979", "7/10/1955", "18/3/1964", "17/2/1954", "25/11/2001", "14/1/1965", "9/8/1957", "10/10/1971", "17/5/1973", "6/3/1958", "21/4/1991", "24/9/1959", "7/3/1958", "15/12/1980", "13/3/1972", "9/4/1979", "28/9/1973", "9/4/1954", "20/8/1995", "28/2/1954", "8/12/1993", "22/7/1981", "6/9/1955", "3/12/1962", "2/7/1998"];
    let mut res: Vec<&str> = Vec::new();
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..99999999).to_string();
    for i in necesidades {
        match i {
            Campos::Nombre => { res.push(nombres.choose(&mut rng).unwrap())}
            Campos::Telefono => { res.push(telefonos.choose(&mut rng).unwrap())}
            Campos::DNI => {res.push(dnis.choose(&mut rng).unwrap())}
            Campos::Fecha => {res.push(fechas.choose(&mut rng).unwrap())}
            Campos::Numero => {
                res.push(&num);
            }
        }
    }
    let mut resutl = String::new();
    for i in &res{
        resutl.push('\"');
        for c in i.chars() {
            resutl.push(c);
        }
        resutl.push('\"');
        if i.to_string() != res[res.len() - 1].to_string() {
            resutl.push(',');
            resutl.push(' ');
        }

    }

    return resutl;



}

