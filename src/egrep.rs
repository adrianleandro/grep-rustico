#![allow(dead_code)]
use std::{
    fs::File,
    io::{self, BufRead, Read},
    path::Path,
};
mod regex;

const COLOR_ROJO: &str = "\x1b[31m";
const COLOR_STD: &str = "\x1b[0m";

pub fn buscar<'a>(reg_ex: &'a str, archivo: &'a str) -> Result<(), &'a str> {
    let expresion_regular_wrapped: Result<regex::Regex, &str> = regex::Regex::new(&reg_ex);
    //si el archivo no existe devolver err(el archivo no existe to owned)
    //por cada linea del archivo leerla <---------
    //regex_esta_en_linea(&regex, linea)         |
    //si el filtro da some printear la linea     |
    //pasar a siguiente linea --------------------
    let expresion_regular = match expresion_regular_wrapped {
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

    loop {
        match cursor.read_line(&mut linea_actual) {
            Ok(0) => {
                return Ok(());
            }
            Ok(_) => {
                match &expresion_regular.testear_linea(&linea_actual){
                    Ok(true) => {print!("{linea_actual}")},
                    Ok(false) => (),
                    Err(e) => println!("{e}"),
                }
                linea_actual.clear();
            }
            Err(_) => {
                return Err("Error al leer el archivo");
            }
        }
        //return Ok(());
    }
}

fn resaltar(inicio_match: usize, fin_match: usize, linea: &String) {
    println!(
        "{}{}{}{}{}",
        &linea[..inicio_match],
        COLOR_ROJO,
        &linea[inicio_match..(inicio_match + fin_match)],
        COLOR_STD,
        &linea[(inicio_match + fin_match)..]
    );
}
