macro_rules! impl_operator {
    ($( $name:ident => $render:expr ),+ $(,)?) => {
        $(
            #[derive(Clone, Copy, Debug)]
            pub struct $name;

            impl $name {
                pub fn value() -> &'static str {
                    stringify!($name)
                }

                pub fn debug(&self) -> String {
                    format!("{}", stringify!($name))
                }

                pub fn render(&self) -> String {
                    $render.to_string()
                }
            }
        )+
    };
}

impl_operator!(
    Addition => "+",
    Subtraction => "-",
    Multiplication => "*",
    Division => "/",
    EqualTo => "==",
    NotEqualTo => "!=",
    GreaterThan => ">",
    GreaterThanOrEqualTo => ">=",
    LessThan => "<",
    LessThanOrEqualTo => "<=",
    LogicalAnd => "&&",
    LogicalOr => "||",
    Not => "!",
    Negate => "-",
    Concatenation => "++",
    Implication => "=>",
    Update => ":=",
);

#[derive(Clone, Debug)]
pub enum Operator {
    Addition(Addition),
    Concatenation(Concatenation),
    EqualTo(EqualTo),
    GreaterThan(GreaterThan),
    GreaterThanOrEqualTo(GreaterThanOrEqualTo),
    Division(Division),
    Implication(Implication),
    LessThan(LessThan),
    LessThanOrEqualTo(LessThanOrEqualTo),
    LogicalAnd(LogicalAnd),
    LogicalOr(LogicalOr),
    Multiplication(Multiplication),
    NotEqualTo(NotEqualTo),
    Subtraction(Subtraction),
    Update(Update),
    Not(Not),
    Negate(Negate),
}

impl Operator {
    pub fn render(&self) -> String {
        match self {
            Operator::Addition(x) => x.render(),
            Operator::Concatenation(x) => x.render(),
            Operator::EqualTo(x) => x.render(),
            Operator::GreaterThan(x) => x.render(),
            Operator::GreaterThanOrEqualTo(x) => x.render(),
            Operator::Division(x) => x.render(),
            Operator::Implication(x) => x.render(),
            Operator::LessThan(x) => x.render(),
            Operator::LessThanOrEqualTo(x) => x.render(),
            Operator::LogicalAnd(x) => x.render(),
            Operator::LogicalOr(x) => x.render(),
            Operator::Multiplication(x) => x.render(),
            Operator::NotEqualTo(x) => x.render(),
            Operator::Subtraction(x) => x.render(),
            Operator::Update(x) => x.render(),
            Operator::Not(x) => x.render(),
            Operator::Negate(x) => x.render(),
        }
    }

    pub fn is_unary(&self) -> bool {
        matches!(self, Operator::Not(_) | Operator::Negate(_))
    }

    pub fn is_binary(&self) -> bool {
        !self.is_unary()
    }

}