use crate::function::{Function, Signature};
use crate::instr::{Expression, Instr};

use wasamere::module::ParsedModule;
use wasamere::section::Export;

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub(crate) funcs: Vec<Function>,
    pub(crate) exports: Vec<Export>,
    // mems:
    // globals:
}

impl Module {
    pub fn from_bytes(bytes: &[u8]) -> Module {
        let parsed_module = ParsedModule::from_bytes(bytes);

        let types = &parsed_module.types.0;
        let funcsec = &parsed_module.funcs.0;
        let bodies = &parsed_module.code.0;

        let mut functions: Vec<Function> = Vec::new();

        for (typeidx, body) in funcsec.into_iter().zip(bodies.into_iter()) {
            let idx: u32 = (*typeidx).into();

            let functype = &types[idx as usize];
            let signature = Signature::from(functype.clone());
            let locals = body.0.clone();
            let code = body.1.clone();

            let function = Function::new(signature, locals, code);
            functions.push(function);
        }

        // for
        let exports = &parsed_module.exports.0;

        Module {
            funcs: functions,
            exports: exports.clone(),
        }
    }
}
