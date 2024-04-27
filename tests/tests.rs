use grep_rustico::egrep::buscar;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn buscar_con_regex_vacia() {
        buscar("", "tests/test.txt").unwrap();
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
    fn comodin_y_asterisco() {
        let resultado = buscar("de.*fg", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 1);
        assert_eq!(
            resultado[0],
            "se prueba el patron con punto y asterisco: de44444fg\n"
        );
    }

    #[test]
    fn caracter_escapeado() {
        let resultado = buscar("abc\\|d", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0], "ejemplo de abc|deeeeeeef\n");
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
        assert_eq!(
            resultado[0],
            "la b podria repetirse de 2 a 4 veces abbbcd\n"
        );
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
        assert_eq!(
            resultado[0],
            "la b podria repetirse de 2 a 4 veces abbbcd\n"
        );
    }

    #[test]
    fn opciones() {
        let resultado = buscar("a[bc]d", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 1);
        assert_eq!(
            resultado[0],
            "se prueba la eleccion entre la b o la c: abd\n"
        );
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
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0], "en un proyecto nuevo de rust se genera un programa que imprime hola4mundo por la pantalla principal\n");
    }

    #[test]
    fn clase_upper_dos_veces() {
        let resultado = buscar("[[:upper:]]ascal[[:upper:]]ase", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 1);
        assert_eq!(
            resultado[0],
            "La notacion PascalCase combina las palabras directamente\n"
        );
    }

    #[test]
    fn clase_alfabetico() {
        let resultado = buscar("[[:alpha:]] es .* y alfabetico", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0], "el caracter k es alfanumerico y alfabetico\n");
    }

    #[test]
    fn clase_alfanumerico() {
        let resultado =
            buscar("el caracter [[:alnum:]] no es un simbolo", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 1);
        assert_eq!(
            resultado[0],
            "el caracter 1 no es un simbolo si no que es un numero\n"
        );
    }

    #[test]
    fn clase_digito() {
        let resultado = buscar("hola[[:digit:]]+ mundo", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0], "hola321 mundo\n");
    }

    #[test]
    fn clase_puntuacion() {
        let resultado = buscar("frutas[[:punct:]]", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0], "frutas: banana anana mango\n");
    }

    #[test]
    fn clases_upper_y_lower() {
        let resultado = buscar(
            "[[:upper:]][[:lower:]]+[[:upper:]][[:lower:]]+",
            "tests/test.txt",
        )
        .unwrap();
        assert_eq!(resultado.len(), 1);
        assert_eq!(
            resultado[0],
            "La notacion PascalCase combina las palabras directamente\n"
        );
    }

    #[test]
    fn varias_clases_con_operadores() {
        let resultado = buscar("[[:upper:]]+[[:space:]]*[[:upper:]]+", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 2);
        assert_eq!(resultado[0], "HOLA MUNDO\n");
        assert_eq!(resultado[1], "CHAU MUNDO\n");
    }

    #[test]
    fn excluir_clase() {
        let resultado = buscar("hola[^[:space:]]mundo", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0], "en un proyecto nuevo de rust se genera un programa que imprime hola4mundo por la pantalla principal\n");
    }

    #[test]
    fn opciones_incluidas() {
        let resultado = buscar("opciones [ab]", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 2);
        assert_eq!(resultado[0], "prueba de opciones a\n");
        assert_eq!(resultado[1], "prueba de opciones b\n");
    }

    #[test]
    fn opciones_excluidas() {
        let resultado = buscar("opciones [^ab]", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0], "prueba de opciones o\n");
    }

    #[test]
    fn operador_empezar_desde_principio() {
        let resultado = buscar("^frutas:", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0], "frutas: banana anana mango\n");
    }

    #[test]
    fn operador_empezar_desde_fin() {
        let resultado = buscar("fin$", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0], "es el fin");
    }

    #[test]
    fn operador_desde_principio_a_fin_matchea_linea_entera() {
        let resultado = buscar("^el caracter 2 es alfanumerico pero no alfabetico$", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0], "el caracter 2 es alfanumerico pero no alfabetico\n");
    }

    #[test]
    fn operador_desde_principio_a_fin_no_matchea_principio() {
        let resultado = buscar("^mango$", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 0);
    }

    #[test]
    fn operador_desde_principio_a_fin_no_matchea_fin() {
        let resultado = buscar("^frutas:$", "tests/test.txt").unwrap();
        assert_eq!(resultado.len(), 0);
    }
}
