#[derive(Debug, Clone)]
pub enum Ast {
    Int(i32),
    Symbol(String),
    List(Vec<Ast>),
    Var(String, Box<Ast>),
    Function(String, Vec<String>, Vec<Ast>),
    Null,
}
