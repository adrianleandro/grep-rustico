use grep_rustico::egrep::buscar;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn buscar_con_regex_vacia() {
        buscar("", "tests/archivo_inexistente.txt").unwrap();
    }

    #[test]
    #[should_panic]
    fn buscar_en_archivo_inexistente() {
        buscar("prueba", "").unwrap();
    }

    #[test]
    fn literal() {
        let resultado = buscar("numero divisible", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0], "el 8 es un numero divisible por 2\n");
    }

    #[test]
    fn comodin() {
        let resultado = buscar("ab.cd", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0], "se prueba el patron de punto: abvcd\n");
    }

    #[test]
    fn signo_pregunta() {
        let resultado = buscar("xy?z", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 2);
        assert_eq!(resultado[0], "esta linea tiene xyz\n");
        assert_eq!(resultado[1], "esta linea tiene xz\n");
    }

    #[test]
    fn llaves_exacto() {
        let resultado = buscar("ab{3}cd", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0], "la b podria repetirse de 2 a 4 veces abbbcd\n");
    }


    #[test]
    fn llaves_minimo() {
        let resultado = buscar("de{4,}f", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0], "ejemplo de abc|deeeeeeef\n");
    }

    #[test]
    fn llaves_maximo() {
        let resultado = buscar("de{,7}f", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0], "ejemplo de abc|deeeeeeef\n");
    }

    #[test]
    fn llaves_rango() {
        let resultado = buscar("ab{2,4}cd", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0], "la b podria repetirse de 2 a 4 veces abbbcd\n");
    }

    #[test]
    fn clase_space() {
        let resultado = buscar("hola[[:space:]]mundo", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0], "en un proyecto nuevo de rust se genera un programa que imprime hola mundo por la pantalla principal\n");
    }

    #[test]
    fn clase_digit() {
        let resultado = buscar("hola[[:digit:]]mundo", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 2);
        assert_eq!(resultado[0], "hola321 mundo\n");
        assert_eq!(resultado[1], "en un proyecto nuevo de rust se genera un programa que imprime hola4mundo por la pantalla principal\n");
    }
}
