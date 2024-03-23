use std::env::args as argv;
mod egrep;
fn main() {
    let args: Vec<String> = argv().collect();
    
    if argv().count() != 3 {
        if let Some(nombre_programa) = args.get(0) {
            eprintln!("Uso: {} [expresión regular] [archivo]", nombre_programa);
        }
        return;
    }

    match egrep::egrep(&args[1], &args[2]){
        Ok(()) => {},
        Err(e) => {
            eprint!("{e}");
        }
    }
    return;
}