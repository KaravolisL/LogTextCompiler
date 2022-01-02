
use std::fs;
use clap::Parser;

mod emitter;
mod lexer;
mod parse;
mod code_generation;
#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
    /// File containing source code to compile
    #[clap(short, long)]
    source_file: String,

    /// Name of the output file
    #[clap(short, long, default_value="Program.out")]
    out: String
}

fn main() {
    let args = Args::parse();

    let source_code = fs::read_to_string(args.source_file)
                                .expect("File doesn't exist");
    
    let lexer = lexer::Lexer::new(source_code);
    let emitter = emitter::Emitter::new(&args.out);
    let mut parser = parse::Parser::new(lexer, emitter);

    parser.program();
}
