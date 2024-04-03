#![allow(dead_code)]
use std::{
    fs::File,
    io::{self, BufRead, Read},
    path::Path,
};
mod regex;

const COLOR_ROJO: &str = "\x1b[31m";
const COLOR_STD: &str = "\x1b[0m";

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
                    Ok(true) => {
                        print!("{linea_actual}");
                        ocurrencias.push(linea_actual.to_owned());
                    }
                    Ok(false) => (),
                    Err(e) => println!("{e}"),
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
