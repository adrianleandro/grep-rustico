use std::{
    fs::File,
    io::{self, BufRead, Read},
    path::Path,
};
mod regex;

/// Recibe una cadena de caracteres a partir de la cual se crea la expresión regular y el path a un archivo.  
/// Busca las ocurrencias de la expresión regular en cada linea del archivo y las muestra en pantalla.  
/// Devuelve un vector con todas las lineas en donde se encontró la expresión regular.  
/// Puede dar error en los siguientes casos:  
/// * expresion regular inválida (ya sea porque esta vacía o bien cuando hay llaves sin cerrar)
/// * no se pudo abrir el archivo, por ruta invalida o por error de apertura
/// * error de lectura en el archivo
/// Además, en caso de que una de las lineas contenga algún caracter que no pertenezca al formato ascii, devolvera ese error por pantalla y continuará evaluando la linea siguiente
pub fn buscar<'a>(reg_ex: &'a str, archivo: &'a str) -> Result<Vec<String>, &'a str> {
    let expresion_regular = match regex::Regex::new(&reg_ex) {
        Ok(expresion_regular) => expresion_regular,
        Err(e) => {
            return Err(e);
        }
    };
    let path = Path::new(archivo);

    let mut file = match File::open(&path) {
        Err(_) => return Err("No se pudo abrir el archivo"),
        Ok(file) => file,
    };

    let mut contenido = String::new();
    if let Err(_) = file.read_to_string(&mut contenido) {
        return Err("Error al leer el archivo");
    };

    let mut cursor = io::Cursor::new(contenido);
    let mut linea_actual = String::new();

    let mut ocurrencias: Vec<String> = Vec::new();
    loop {
        match cursor.read_line(&mut linea_actual) {
            Ok(0) => {
                return Ok(ocurrencias);
            }
            Ok(_) => {
                match &expresion_regular.testear_linea(&linea_actual) {
                    Ok((0,0)) => (),
                    Ok((start,end)) => {
                        resaltar(*start, *end, &linea_actual);
                        ocurrencias.push(linea_actual.to_owned());
                    }
                    Err(e) => eprintln!("{e}"),
                }
                linea_actual.clear();
            }
            Err(_) => {
                return Err("Error al leer el archivo");
            }
        }
        //return Ok(ocurrencias);
    }
}

fn resaltar(comienzo_match: usize, fin_match: usize, linea: &String) {
    print!("{}{}{}{}{}", &linea[..comienzo_match],"\x1b[31m", &linea[comienzo_match..fin_match], "\x1b[0m", &linea[fin_match..]);
}