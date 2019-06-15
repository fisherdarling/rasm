use crate::function::{Function, Signature};
use crate::instr::{Expression, Instr};

use wasamere::module::ParsedModule;
use wasamere::section::{Export, Section};

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub(crate) funcs: Vec<Function>,
    pub(crate) exports: Vec<Export>,
    // pub(crate) mems: Vec<
    // globals:
}

impl Module {
    pub fn from_bytes(bytes: &[u8]) -> Module {
        let parsed_module = ParsedModule::parse_bytes(bytes);

        let types = &parsed_module.sections().iter().find_map(Section::map_type).cloned().unwrap_or_default().0;
        let funcsec = &parsed_module.sections().iter().find_map(Section::map_func).cloned().unwrap_or_default().0;
        let bodies = &parsed_module.sections().iter().find_map(Section::map_code).cloned().unwrap_or_default().0;

        let mut functions: Vec<Function> = Vec::new();

        for (typeidx, body) in funcsec.into_iter().zip(bodies.into_iter()) {
            let idx: u32 = (*typeidx).into();

            let functype = &types[idx as usize];
            let signature = Signature::from(functype.clone());
            let locals = body.0.clone();
            let code = body.1.clone().flatten();

            let function = Function::new(signature, locals, code);
            functions.push(function);
        }

        let exports = &parsed_module.sections().iter().find_map(Section::map_export).cloned().unwrap_or_default().0;

        Module {
            funcs: functions,
            exports: exports.clone(),
        }
    }
}
