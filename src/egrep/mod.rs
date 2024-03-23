mod archivo;

const COLOR_ROJO: &str = "\x1b[31m";
const COLOR_STD: &str = "\x1b[0m";

pub fn egrep(regex: &String, archivo: &String) -> Result<(), String>{
   //abrir archivo
   //si el archivo no existe devolver err(el archivo no existe to owned)
   //por cada linea del archivo leerla <---------
   //regex_esta_en_linea(&regex, linea)         |
   //si el filtro no da error printear la linea |
   //pasar a siguiente linea --------------------
   Ok(())
}

fn resaltar(regex: &String, linea: &String) {
    if let Some(index) = linea.find(regex) {
        println!("{}{}{}{}{}", &linea[..index],COLOR_ROJO, &linea[index..(index + regex.len())], COLOR_STD, &linea[(index + regex.len())..]);
    }
}