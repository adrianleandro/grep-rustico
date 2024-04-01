#![allow(dead_code)]
use core::arch;
use std::{
    fs::File,
    io::{self, BufRead, Read},
    path::{Path, PathBuf},
};
mod regex;

const COLOR_ROJO: &str = "\x1b[31m";
const COLOR_STD: &str = "\x1b[0m";

pub fn buscar<'a>(reg_ex: &'a str, archivo: &'a str) -> Result<Vec<String>, &'a str> {
    let mut ocurrencias: Vec<String> = Vec::new();
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

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(_) => return Err("Error al leer el archivo"),
        Ok(_) => (),
    };

    let mut cursor = io::Cursor::new(s);
    let mut buf = String::new();

    loop {
        match cursor.read_line(&mut buf) {
            Ok(0) => {
                return Ok(ocurrencias);
            }
            Ok(_) => {
                print!("{buf}");
                buf.clear()
            }
            Err(_) => {
                return Err("Error al leer el archivo");
            }
        }
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
