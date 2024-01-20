use base::{compile_context::CompileContext, file_cache::FileCache};
use clap::{arg, command, Parser};
use codespan_reporting::term::termcolor::{BufferedStandardStream, ColorChoice};
use std::{env, path::PathBuf};

extern crate driver;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short = 'p',
        long = "path",
        required = true,
        help = "Path to the package to build"
    )]
    path: String,
    #[arg(
        short = 'd',
        long = "emit_irs",
        help = "Emit the AST and MIR to files in the build directory"
    )]
    emit_irs: bool,
    #[arg(
        short = 'r',
        long = "run",
        help = "Run the program immediately after compiling"
    )]
    run_immediately: bool,
}

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");

    let args = Args::parse();
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let build_dir = current_dir.join(".build");
    let emit_irs = args.emit_irs;
    let run_immediately = args.run_immediately;
    let build_source = PathBuf::from(args.path);
    let file_cache = FileCache::empty();

    let mut context = CompileContext {
        file_cache,
        emit_irs,
        build_dir,
        build_pkg: build_source,
        run_immediately,
    };

    // only delete if it is called .build
    if context.build_dir.ends_with(".build") && context.build_dir.exists() {
        std::fs::remove_dir_all(&context.build_dir).expect("Failed to remove build directory");
    }
    std::fs::create_dir_all(&context.build_dir).expect("Failed to create build directory");

    let mut compiler = driver::Compiler::new(&mut context);
    let res = compiler.compile();

    let mut writer = BufferedStandardStream::stderr(ColorChoice::Always);
    let reporting_config = codespan_reporting::term::Config::default();

    match res {
        Ok(_) => {}
        Err(e) => {
            let diagnostic = e.as_diagnostic();

            codespan_reporting::term::emit(
                &mut writer,
                &reporting_config,
                &context.file_cache,
                &diagnostic,
            )
            .unwrap();
        }
    }
}
