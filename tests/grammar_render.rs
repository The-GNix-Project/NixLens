use nix_lens::parser::ast::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_types_rendering() {
        let id = Identifier::new("foo".to_string());
        assert_eq!(id.render(), "foo");

        let int = Integer::new("42".to_string());
        assert_eq!(int.render(), "42");

        let float = Float::new("3.14".to_string());
        assert_eq!(float.render(), "3.14");

    }

    #[test]
    fn test_operators_rendering() {
        assert_eq!(operators::Addition.render(), "+");
        assert_eq!(operators::Subtraction.render(), "-");
        assert_eq!(operators::Multiplication.render(), "*");
        assert_eq!(operators::Division.render(), "/");
        assert_eq!(operators::EqualTo.render(), "==");
        assert_eq!(operators::NotEqualTo.render(), "!=");
        assert_eq!(operators::GreaterThan.render(), ">");
        assert_eq!(operators::GreaterThanOrEqualTo.render(), ">=");
        assert_eq!(operators::LessThan.render(), "<");
        assert_eq!(operators::LessThanOrEqualTo.render(), "<=");
        assert_eq!(operators::LogicalAnd.render(), "&&");
        assert_eq!(operators::LogicalOr.render(), "||");
        assert_eq!(operators::Not.render(), "!");
        assert_eq!(operators::Negate.render(), "-");
        assert_eq!(operators::Concatenation.render(), "++");
        assert_eq!(operators::Update.render(), ":=");
    }

    // Functions
    #[test]
    fn test_function_rendering() {
        let arg = FunctionHeadDestructuredArgument {
            identifier: "arg".to_string(),
            default: Some(Expression::Integer(Integer::new("42".to_string()))),
        };
        assert_eq!(arg.render().unwrap(), "arg ? 42");

        let simple_head = FunctionHeadSimple::new(Identifier::new("arg".to_string()));
        assert_eq!(simple_head.render(), "arg");

        let func = Function::new(
            Expression::Identifier(Identifier::new("arg".to_string())),
            Expression::Identifier(Identifier::new("body".to_string())),
        );
        assert_eq!(func.render().unwrap(), "arg: body");

        let app = FunctionApplication::new(
            Expression::Identifier(Identifier::new("func".to_string())),
            vec![Expression::Identifier(Identifier::new("arg".to_string()))]
        );
        assert_eq!(app.render().unwrap(), "func arg");
    }

    #[test]
    fn test_collections_rendering() {
        let list = List::new(vec![
            Expression::Identifier(Identifier::new("a".to_string())),
            Expression::Identifier(Identifier::new("b".to_string())),
        ]);
        assert_eq!(list.render().unwrap(), "[a, b]");

        let map = AttrSet::new(
            false,
            vec![Expression::BindingKeyValue(BindingKeyValue::new(
                Expression::Identifier(Identifier::new("a".to_string())),
                Expression::Integer(Integer::new("1".to_string())),
            ))],
        );
        assert_eq!(map.render().unwrap(), "{ a = 1; }");

        let rec_map = AttrSet::new(
            true,
            vec![Expression::BindingKeyValue(BindingKeyValue::new(
                Expression::Identifier(Identifier::new("a".to_string())),
                Expression::Integer(Integer::new("1".to_string())),
            ))],
        );
        assert_eq!(rec_map.render().unwrap(), "rec { a = 1; }");
    }

    #[test]
    fn test_scoping_rendering() {
        let with = With::new(
            Expression::Identifier(Identifier::new("pkgs".to_string())),
            Expression::List(List::new(vec![
                Expression::Identifier(Identifier::new("hello".to_string())),
            ])),
        );
        assert_eq!(with.render().unwrap(), "with pkgs; [hello]");
    }

    #[test]
    fn test_bindings_rendering() {
        let inherit = BindingInherit::new(
            None,
            Expression::Identifier(Identifier::new("attr".to_string())),
        );
        assert_eq!(inherit.render().unwrap(), "inherit attr");

        let kv = BindingKeyValue::new(
            Expression::Identifier(Identifier::new("name".to_string())),
            Expression::Identifier(Identifier::new("value".to_string())),
        );
        assert_eq!(kv.render().unwrap(), "name = value;");
    }
}
