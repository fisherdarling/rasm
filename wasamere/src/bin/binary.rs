// use std::fs::;

use env_logger::try_init;
use wasamere::module::ParsedModule;
use wasamere::StructNom;
use wasamere::section::Section;
use wasamere::types::Function;
use wasamere::instr::Instr;

use std::iter;

fn main() {
    let _ = try_init().unwrap();

    let source = include_bytes!("../../examples/large_func.wasm");

    let (rest, module) = ParsedModule::nom(source).unwrap();
    let code = module.sections().iter().find_map(Section::map_code).expect("Unable to find code section").clone();

    for function in code.0.into_iter() {
        let mut acc = Vec::new();

        for instr in function.1.into_iter() {
            flatten(&mut acc, instr.clone());
        }

        println!("Flattend: {:#?}", acc);
    }


    // println!("{:#?}", code);
}



fn flatten(acc: &mut Vec<Instr>, instr: Instr) {
    match instr {
        Instr::Block(res, expr) => {
            acc.push(Instr::BlockMarker);
            expr.0.into_iter().for_each( |i| flatten(acc, i));
        },
        Instr::Loop(res, expr) => {
            acc.push(Instr::LoopMarker);
            expr.0.into_iter().for_each( |i| flatten(acc, i));
        },
        Instr::If(res, e1, e2) => {
            acc.push(Instr::IfMarker);
            acc.push(Instr::ConseqMarker);
            e1.0.into_iter().for_each( |i| flatten(acc, i));
            acc.push(Instr::AlternMarker);
            e2.0.into_iter().for_each( |i| flatten(acc, i));
        },
        non_expr_instr => acc.push(non_expr_instr),
    }
}