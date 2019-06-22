use wabt::script::{Action, Command, CommandKind, ScriptParser, Value as WastValue};

use colored::*;
use structopt::StructOpt;



use std::path::PathBuf;


use yawr::error::Error;
use yawr::runtime::ModuleInstance;
use yawr::types::{Value, WasmResult};

#[derive(Debug, Clone, StructOpt)]
struct Args {
    #[structopt(short = "f", long = "file")]
    pub files: Vec<PathBuf>,
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,
}

fn main() {
    let args = Args::from_args();
    let verbose = args.verbose;

    for file in args.files {
        let bar = "================".purple();
        println!(
            "{} {} {}",
            bar,
            file.to_str().unwrap().bright_yellow().bold(),
            bar
        );

        let file = std::fs::read_to_string(&file).unwrap();

        let mut parser = ScriptParser::<f32, f64>::from_str(&file).unwrap();

        let mut runtime = ModuleInstance::default();

        while let Some(Command { kind, line }) = parser.next().unwrap() {
            match kind {
                CommandKind::Module { module, name } => {
                    // The module is declared as annonymous.
                    assert_eq!(name, None);

                    // Convert the module into the binary representation and check the magic number.
                    let module_binary = module.into_vec();
                    runtime = ModuleInstance::from_bytes(&module_binary).unwrap();

                    println!("[MODULE] Name: {:?}", name);
                }
                CommandKind::AssertReturn { action, expected } => {
                    match action {
                        Action::Invoke { field, args, .. } => {
                            let args = translate_wast_values(args);

                            let res = runtime.invoke(&field, &args);
                            let expected: Vec<WasmResult> = translate_wast_values(expected)
                                .into_iter()
                                .map(WasmResult::from)
                                .collect();

                            if let Ok(result) = res {
                                if expected.is_empty() && result == WasmResult::Unit {
                                    print_passed(line, &field, verbose);
                                } else if expected.len() > 0 && expected[0] == result {
                                    print_passed(line, &field, verbose);
                                } else {
                                    print_failed(line, &field, args, expected, result, verbose);
                                }
                            } else {
                                print_failed_hard(line, &field, args, expected, res, verbose);
                            }
                        }
                        _ => {}
                        // a => println!("[UNSUPPORTED] Action: {:?}", a),
                    }
                }
                _ => {}
                // k => println!("[UNSUPPORTED] Kind: {:?}", k),
            }
        }
    }

    // println!("ModuleInstance: {:?}", runtime);
}

fn print_passed(line: u64, field: &str, verbose: bool) {
    if !verbose {
        println!(
            "[{:04}] {} {:<30}",
            line,
            "[PASSED]".green(),
            &field.white()
        );
    } else {
        println!(
            "[{:04}] {} {:<30}",
            line,
            "[PASSED]".green(),
            &field.white()
        );
    }
}

fn print_failed(
    line: u64,
    field: &str,
    args: Vec<Value>,
    expected: Vec<WasmResult>,
    result: WasmResult,
    verbose: bool,
) {
    if verbose {
        println!(
            r#"[{:04}] {} {:<30} 
                ├──>  Args:     {:?}
                ├──>  Expected: {:?}
                └──>  Received: {:?}"#,
            line,
            "[FAILED]".red(),
            &field.yellow(),
            args,
            expected,
            result
        );
    } else {
        println!("[{:04}] {} {:<30}", line, "[FAILED]".red(), &field.yellow());
    }
}
fn print_failed_hard(
    line: u64,
    field: &str,
    args: Vec<Value>,
    expected: Vec<WasmResult>,
    result: Result<WasmResult, Error>,
    verbose: bool,
) {
    if verbose {
        println!(
            "[{:04}] {} {:<30} Args: {:?}, || Expected: {:?}, || Received: {:?}",
            line,
            "[FAILED]".red().underline().bold(),
            &field.yellow(),
            args,
            expected,
            result
        );
    } else {
        println!(
            "[{:04}] {} {:<30}",
            line,
            "[FAILED]".red().underline().bold(),
            &field.yellow()
        );
    }
}

fn translate_wast_values(args: Vec<WastValue>) -> Vec<Value> {
    args.into_iter()
        .map(|a| match a {
            WastValue::I32(v) => Value::I32(v),
            WastValue::I64(v) => Value::I64(v),
            WastValue::F32(v) => Value::F32(v),
            WastValue::F64(v) => Value::F64(v),
        })
        .collect()
}
