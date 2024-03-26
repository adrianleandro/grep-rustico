
#[cfg(test)]
mod tests{
    #[test]
    fn period(){
        assert_eq!(".", ".");
    }

    #[test]
    fn period_with_asterisc(){
        assert_eq!(".*", ".*");
    }

    #[test]
    fn bracket_expression(){
        assert_eq!("[]", "[]");
    }

    #[test]
    fn curly_bracket_expression(){
        assert_eq!("{}", "{}");
    }
    
}