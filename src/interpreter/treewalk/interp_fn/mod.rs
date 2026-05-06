use std::rc::Rc;

use crate::{
    frontend::parser::ast::{Binding, Expr, Stmt},
    interpreter::treewalk::{Env, interp_expr, types::Value},
};

pub(super) fn interp_fn(name: String, stmts: Vec<&Stmt>, env: Rc<Env>) -> Rc<Value> {
    let mut local_env = Rc::clone(&env);
    for stmt in stmts {
        match stmt {
            Stmt::Let(id, expr) => {
                let val = interp_expr::interp_expr(expr, Rc::clone(&local_env));
                local_env = local_env.extend(id.clone(), val);
            }
            Stmt::Read(type_of_expr, id) => todo!(),
            Stmt::Echo(type_of_expr, expr) => todo!(),
            Stmt::Return(expr) => return interp_expr::interp_expr(expr, Rc::clone(&local_env)),
        }
    }

    panic!("FATAL ERROR: function {} does not return", name)
}

fn bind_args(env: Rc<Env>, params: Vec<Binding>, args: Vec<Expr>) -> Rc<Env> {
    unimplemented!()
}
