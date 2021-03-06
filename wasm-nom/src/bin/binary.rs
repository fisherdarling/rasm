// use std::fs::;

use env_logger::try_init;
use wasm_nom::instr::Instr;
use wasm_nom::module::ParsedModule;
use wasm_nom::section::Section;

use wasm_nom::StructNom;

fn main() {
    let _ = try_init().unwrap();

    let source = include_bytes!("../../../examples/loop2.wasm");

    let (_rest, module) = ParsedModule::nom(source).unwrap();
    let code = module
        .sections()
        .iter()
        .find_map(Section::map_code)
        // .unwrap()
        .expect("Unable to find code section")
        .clone();

    for (i, func) in code.0.iter().enumerate() {
        let mut acc = Vec::new();

        let _test: Vec<Instr> = Vec::new();

        for instr in &(func.1).0 {
            flatten(&mut acc, instr.clone());
        }

        println!("==== FUNC {} ====", i);
        for (i, instr) in acc.iter().enumerate() {
            println!("[{:2?}]: {:?}", i, instr);
        }
    }
}

fn flatten(acc: &mut Vec<Instr>, instr: Instr) {
    match instr {
        Instr::Block(res, expr) => {
            acc.push(Instr::Nop);
            let idx = acc.len() - 1;

            expr.0.into_iter().for_each(|i| flatten(acc, i));

            acc[idx] = Instr::BlockMarker(res, acc.len() - 1);
        }
        Instr::Loop(res, expr) => {
            let idx = acc.len();
            acc.push(Instr::LoopMarker(res, idx));

            expr.0.into_iter().for_each(|i| flatten(acc, i));
        }
        Instr::If(res, e1, e2) => {
            acc.push(Instr::Nop);
            let idx = acc.len() - 1;

            e1.0.into_iter().for_each(|i| flatten(acc, i));
            let first_end = acc.len() - 1;

            e2.0.into_iter().for_each(|i| flatten(acc, i));
            let second_end = acc.len() - 1;

            acc[idx] = Instr::IfMarker(res, first_end, second_end);
        }
        non_expr_instr => acc.push(non_expr_instr),
    }
}
