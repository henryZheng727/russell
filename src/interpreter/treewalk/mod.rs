use std::process::ExitCode;

use crate::frontend::parser::ast::Defn;
use crate::interpreter::treewalk::types::Env;

mod types;

pub fn interp(defns: Vec<Defn>) -> ExitCode {
    for defn in defns {
        match defn {
            Defn::Typedef(_, items) => todo!(),
            Defn::Fn(_, bindings, _, stmts) => todo!(),
        }
    }
    unimplemented!()
}

fn process_global_env(defns: Vec<Defn>) -> (Env, Vec<Defn>) {
    unimplemented!()
}
