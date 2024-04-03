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
    fn period_with_asterisc() {
        assert_eq!(".*", ".*");
    }

    #[test]
    fn bracket_expression() {
        assert_eq!("[]", "[]");
    }

    #[test]
    fn curly_bracket_expression() {
        assert_eq!("{}", "{}");
    }
}
