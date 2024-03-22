use std::env::args as argv;
fn main() {
    let args: Vec<String> = argv().collect();
    
    if argv().count() != 3 {
        if let Some(nombre_programa) = args.get(0) {
            eprintln!("Uso: {} [expresión regular] [archivo]", nombre_programa);
        }
        return;
    }

    //let archivo = abrir_archivo(args.get(2));
    //egrep(args.get(1), archivo);
    return;
}