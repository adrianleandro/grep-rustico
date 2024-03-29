mod regex;
const COLOR_ROJO: &str = "\x1b[31m";
const COLOR_STD: &str = "\x1b[0m";

pub fn buscar(regex: &String, archivo: &String) -> Result<Vec<String>, String>{
   let mut ocurrencias: Vec<String> = Vec::new();
    //abrir archivo
   //si el archivo no existe devolver err(el archivo no existe to owned)
   //por cada linea del archivo leerla <---------
   //regex_esta_en_linea(&regex, linea)         |
   //si el filtro da some printear la linea     |
   //pasar a siguiente linea --------------------
   ocurrencias.push("hola".to_owned());
   resaltar(&ocurrencias[0], &"hola que tal".to_owned());
   Ok(ocurrencias)
}

fn resaltar(matcheado: &String, linea: &String) {
    if let Some(comienzo_matcheado) = linea.find(matcheado) {
        let fin_matcheado: usize = matcheado.len();
        println!("{}{}{}{}{}", &linea[..comienzo_matcheado],COLOR_ROJO, &linea[comienzo_matcheado..(comienzo_matcheado + fin_matcheado)], COLOR_STD, &linea[(comienzo_matcheado + fin_matcheado)..]);
    }
}