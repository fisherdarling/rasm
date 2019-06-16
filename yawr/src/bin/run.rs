use structopt::StructOpt;

use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

use yawr::runtime::Runtime;
use yawr::types::Value;

#[derive(Debug, Clone, StructOpt)]
struct Args {
    #[structopt(short = "b", long = "bin")]
    pub bin: PathBuf,
    #[structopt(short = "f", long = "invoke")]
    pub func: String,
    #[structopt(short = "a")]
    pub args: Vec<i64>,
}

fn main() {
    let args = Args::from_args();
    println!("{:?}", args);

    let _ = env_logger::try_init().unwrap();

    let mut bytes = Vec::new();
    File::open(args.bin).unwrap().read_to_end(&mut bytes).unwrap();

    let mut runtime = Runtime::from_bytes(&bytes);

    let mut func_args = Vec::new();

    for value in args.args {
        func_args.push(Value::I64(value));
    } 

    let start = Instant::now();
    println!("{}({:?})", args.func, func_args);
    let res = runtime.invoke(args.func, &func_args).unwrap();
    println!("[{:?}]: {:?}", start.elapsed(), res);
}