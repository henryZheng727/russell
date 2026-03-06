pub enum Defn {
    Fn(String, Vec<Binding>, Type, Vec<Stmnt>),
}

pub enum Stmnt {
    Let(Type, Expr),
    Read(Type, Expr),
    Echo(Type, Expr),
    Return(Expr),
    Invalid,
}

pub enum Expr {
    // atomic expressions
    Int(i64),
    Float(f64),
    Bool(bool),

    // idents
    Id(String),

    // closures
    Fn(Binding, Box<Expr>), // fn ( <binding> ) -> <expr>

    // unary operators
    Neg(Box<Expr>),  // - <exp>
    Bang(Box<Expr>), // ! <exp>

    // function calls
    Call(Box<Expr>, Box<Expr>), // <left>(<right>)

    // binary operators
    Plus(Box<Expr>, Box<Expr>),    // <left> + <right>
    Minus(Box<Expr>, Box<Expr>),   // <left> - <right>
    Mult(Box<Expr>, Box<Expr>),    // <left> * <right>
    Div(Box<Expr>, Box<Expr>),     // <left> / <right>
    Pipe(Box<Expr>, Box<Expr>),    // <left> |> <right>
    Less(Box<Expr>, Box<Expr>),    // <left> < <right>
    LessEq(Box<Expr>, Box<Expr>),  // <left> <= <right>
    Greater(Box<Expr>, Box<Expr>), // <left> > <right>
    GreaterEq(Box<Expr>, Box<Expr>), // <left> >= <right>
    Eq(Box<Expr>, Box<Expr>),      // <left> == <right>
    NotEq(Box<Expr>, Box<Expr>),   // <left> != <right>
    Or(Box<Expr>, Box<Expr>),      // <left> || <right>
    And(Box<Expr>, Box<Expr>),     // <left> && <right>

    // if expressions
    If(Box<Expr>, Box<Expr>, Box<Expr>), // if <1> then <2> else <3>
}

pub enum Type {
    I64,
    F64,
    Bool,
    TypeId(String),
    Fn(Box<Type>, Box<Type>),
}

pub struct Binding {
    id: String,
    typ: Type,
}
