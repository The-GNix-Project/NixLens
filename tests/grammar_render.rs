use nix_lens::parser::grammar::*;

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a basic span NOT A TEST
    fn span() -> Span {
        Span::new(Position::new(1, 1), Position::new(1, 1))
    }

    #[test]
    fn test_simple_types_rendering() {
        let id = Identifier::new("foo".to_string());
        assert_eq!(id.render(), "foo");

        let int = Integer::new("42".to_string());
        assert_eq!(int.render(), "42");

        let float = Float::new("3.14".to_string());
        assert_eq!(float.render(), "3.14");

        let err = Error::new("test error".to_string());
        assert!(matches!(Expression::Error(err).render(), Err(_)));
    }

    #[test]
    fn test_operators_rendering() {
        assert_eq!(Addition.render(), "+");
        assert_eq!(Subtraction.render(), "-");
        assert_eq!(Multiplication.render(), "*");
        assert_eq!(Division.render(), "/");
        assert_eq!(EqualTo.render(), "==");
        assert_eq!(NotEqualTo.render(), "!=");
        assert_eq!(GreaterThan.render(), ">");
        assert_eq!(GreaterThanOrEqualTo.render(), ">=");
        assert_eq!(LessThan.render(), "<");
        assert_eq!(LessThanOrEqualTo.render(), "<=");
        assert_eq!(LogicalAnd.render(), "&&");
        assert_eq!(LogicalOr.render(), "||");
        assert_eq!(Not.render(), "!");
        assert_eq!(Negate.render(), "-");
        assert_eq!(Concatenation.render(), "++");
        assert_eq!(Update.render(), ":=");
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
            vec![Expression::Identifier(Identifier::new("arg".to_string()))],
            span(),
        );
        assert_eq!(app.render().unwrap(), "func arg");
    }

    #[test]
    fn test_string_components_rendering() {
        let interp = PartInterpolation::new(
            Expression::Identifier(Identifier::new("var".to_string())),
            span(),
        );
        assert_eq!(interp.render().unwrap(), "${var}");

        let raw = PartRaw::new("text".to_string());
        assert_eq!(raw.render(), "text");
    }

    #[test]
    fn test_control_flow_rendering() {
        let bin_op = BinaryOperation::new(
            Expression::Identifier(Identifier::new("a".to_string())),
            Operator::Addition(Addition),
            Expression::Identifier(Identifier::new("b".to_string())),
        );
        assert_eq!(bin_op.render().unwrap(), "(a + b)");

        let assert = Assert::new(
            Expression::Identifier(Identifier::new("cond".to_string())),
            Expression::Identifier(Identifier::new("value".to_string())),
        );
        assert_eq!(assert.render().unwrap(), "assert cond == cond");

        let has_attr = HasAttribute::new(
            Expression::Identifier(Identifier::new("set".to_string())),
            vec![Expression::Identifier(Identifier::new("attr".to_string()))],
        );
        assert_eq!(has_attr.render().unwrap(), "set.attr");

        let if_expr = IfThenElse::new(
            Expression::Identifier(Identifier::new("cond".to_string())),
            Expression::Identifier(Identifier::new("a".to_string())),
            Expression::Identifier(Identifier::new("b".to_string())),
        );
        assert_eq!(if_expr.render().unwrap(), "if cond then a else b");

        let let_in = LetIn::new(
            vec![Expression::BindingKeyValue(BindingKeyValue::new(
                Expression::Identifier(Identifier::new("x".to_string())),
                Expression::Integer(Integer::new("1".to_string())),
            ))],
            Expression::Identifier(Identifier::new("x".to_string())),
        );
        assert_eq!(let_in.render().unwrap(), "let x = 1; in x");
    }

    #[test]
    fn test_collections_rendering() {
        let list = List::new(vec![
            Expression::Identifier(Identifier::new("a".to_string())),
            Expression::Identifier(Identifier::new("b".to_string())),
        ]);
        assert_eq!(list.render().unwrap(), "[a, b]");

        let map = Map::new(
            false,
            vec![Expression::BindingKeyValue(BindingKeyValue::new(
                Expression::Identifier(Identifier::new("a".to_string())),
                Expression::Integer(Integer::new("1".to_string())),
            ))],
        );
        assert_eq!(map.render().unwrap(), "{ a = 1; }");

        let rec_map = Map::new(
            true,
            vec![Expression::BindingKeyValue(BindingKeyValue::new(
                Expression::Identifier(Identifier::new("a".to_string())),
                Expression::Integer(Integer::new("1".to_string())),
            ))],
        );
        assert_eq!(rec_map.render().unwrap(), "rec { a = 1; }");
    }

    #[test]
    fn test_paths_uris_rendering() {
        let path = Path::new(vec![
            Expression::PartRaw(PartRaw::new("path".to_string())),
            Expression::PartRaw(PartRaw::new("to".to_string())),
        ]);
        assert_eq!(path.render().unwrap(), "path/to");

        let uri = Uri::new("https://example.com".to_string());
        assert_eq!(uri.render(), "https://example.com");

        let search_path = SearchNixPath::new("nixpkgs".to_string());
        assert_eq!(search_path.render(), "<nixpkgs>");
    }

    #[test]
    fn test_strings_rendering() {
        let nix_str = NixString::new(vec![
            Expression::PartRaw(PartRaw::new("text ".to_string())),
            Expression::PartInterpolation(PartInterpolation::new(
                Expression::Identifier(Identifier::new("var".to_string())),
                span(),
            )),
        ]);
        assert_eq!(nix_str.render().unwrap(), "text ${var}");

        let indented_str = IndentedString::new(vec![
            Expression::PartRaw(PartRaw::new("multiline\n".to_string())),
            Expression::PartInterpolation(PartInterpolation::new(
                Expression::Identifier(Identifier::new("var".to_string())),
                span(),
            )),
        ]);
        assert_eq!(indented_str.render().unwrap(), "multiline\n${var}");
    }

    #[test]
    fn test_property_access_rendering() {
        let prop_access = PropertyAccess::new(
            Expression::Identifier(Identifier::new("obj".to_string())),
            vec![Expression::Identifier(Identifier::new("attr".to_string()))],
            None,
        );
        assert_eq!(prop_access.render().unwrap(), "obj.attr");

        let unary_op = UnaryOperation::new(
            Operator::Not(Not),
            Expression::Identifier(Identifier::new("a".to_string())),
        );
        assert_eq!(unary_op.render().unwrap(), "(!a)");
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
