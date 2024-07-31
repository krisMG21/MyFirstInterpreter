use crate::expr::Expr;
use crate::utils;

#[derive(Debug, PartialEq)]
pub struct BindingDef{
    name: String,
    value: Expr,
}

impl BindingDef{
    pub fn new(s: &str) -> (&str, Self){
        let s = utils::tag("let", s);

        let (s, _) = utils::extract_whitespace(s);
        let (s, name) = utils::extract_ident(s);
        let (s, _) = utils::extract_whitespace(s);
        
        let s = utils::tag("=", s);

        let (s, _) = utils::extract_whitespace(s);
        let (s, val) = Expr::new(s);

        (
            s,
            Self{
                name : name.to_string(),
                value : val
            }
        )
    }
}


#[cfg(test)]
mod test{
    use super::*;
    use crate::expr::{Number, Op};

    #[test]
    fn parse_binding_def(){
        assert_eq!(
            BindingDef::new("let a = 1/2"),
            (
                "",
                BindingDef{
                    name : "a".to_string(),
                    value : {
                        Expr { lhs: Number(1), rhs: Number(2), op: Op::Div}
                    }
                }
            )
        );
    }
}