use std::rc::Rc;

use crate::{
    frontend::parser::ast::{Binding, Expr, Stmt},
    interpreter::treewalk::{Env, Fn, interp_expr, types::Value},
};

pub(super) fn interp_fn(fn_def: Fn, env: Rc<Env>, args: Vec<Expr>) -> Rc<Value> {
    let mut local_env = Rc::clone(&env);
    for stmt in fn_def.statements {
        match stmt {
            Stmt::Let(id, expr) => todo!(),
            Stmt::Read(type_of_expr, id) => todo!(),
            Stmt::Echo(type_of_expr, expr) => todo!(),
            Stmt::Return(expr) => return interp_expr::interp_expr(expr, Rc::clone(&local_env)),
        }
    }

    panic!("FATAL ERROR: function {} does not return", fn_def.name)
}

fn bind_args(env: Rc<Env>, params: Vec<Binding>, args: Vec<Expr>) -> Rc<Env> {
    unimplemented!()
}
