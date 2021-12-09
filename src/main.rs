use std::collections::*;
use std::thread::spawn;

use crate::file::*;

use argh::*;

use rustnutlib::*;
use rustnutlib::console::*;

use fcpeg::FCPEGError;

use rustnutc::compiler::*;

type ChescCommandResult<T> = Result<T, ChescCommandError>;

pub enum ChescCommandError {
    Unknown {},
    CompilerError { err: CompilerError },
    FCPEGError { err: FCPEGError },
    FileError { err: FileError },
}

impl ConsoleLogger for ChescCommandError {
    fn get_log(&self) -> ConsoleLog {
        return match self {
            ChescCommandError::Unknown {} => log!(Error, "unknown"),
            ChescCommandError::FCPEGError { err } => err.get_log(),
            ChescCommandError::CompilerError { err } => err.get_log(),
            ChescCommandError::FileError { err } => err.get_log(),
        };
    }
}

trait SubCommandProcessor {
    fn proc(&self) -> ChescCommandResult<()>;
}

/// chesc command
#[derive(FromArgs, PartialEq)]
struct TopLevelCommand {
    #[argh(subcommand)]
    subcommand: SubLevelCommand,
}

impl SubCommandProcessor for TopLevelCommand {
    fn proc(&self) -> ChescCommandResult<()> {
        return self.subcommand.proc();
    }
}

/// subcommands for chesc command
#[derive(FromArgs, PartialEq)]
#[argh(subcommand)]
enum SubLevelCommand {
    Cmp(CmpSubcommand),
}

impl SubCommandProcessor for SubLevelCommand {
    fn proc(&self) -> ChescCommandResult<()> {
        return match self {
            SubLevelCommand::Cmp(subcommand) => subcommand.proc(),
        };
    }
}

/// cmp subcommand
#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "cmp")]
struct CmpSubcommand {
    /// input file paths
    #[argh(option, short = 'i')]
    input: String,

    /// output file paths
    #[argh(option, short = 'o')]
    output: String,

    /// reference file paths
    #[argh(option, short = 'l')]
    lib: Option<String>,
}

impl SubCommandProcessor for CmpSubcommand {
    fn proc(&self) -> ChescCommandResult<()> {
        let start_time = std::time::Instant::now();

        let mut lib_dir_paths = match &self.lib {
            Some(v) => v.split(";").collect::<Vec<&str>>(),
            None => vec![],
        };

        // メインソースファイルを持つディレクトリを参照先に指定する
        let abs_main_src_path = match FileMan::get_parent_dir_path(&self.input) {
            Ok(v) => v.unwrap(),
            Err(e) => return Err(ChescCommandError::FileError { err: e }),
        };

        lib_dir_paths.push(abs_main_src_path.to_str().unwrap());

        // todo: 環境変数 CHES_HOME の値を利用する
        let fcpeg_file_path = "src/root/Ches_1/rustnut/compiler/1.0.0/lib/fcpeg/syntax.fcpeg".to_string();
        let mut lib_fcpeg_file_map = HashMap::<String, String>::new();
        lib_fcpeg_file_map.insert("Expr".to_string(), "src/root/Ches_1/rustnut/compiler/1.0.0/lib/fcpeg/expr.fcpeg".to_string());
        lib_fcpeg_file_map.insert("Misc".to_string(), "src/root/Ches_1/rustnut/compiler/1.0.0/lib/fcpeg/misc.fcpeg".to_string());

        let mut cmp = match Compiler::load(fcpeg_file_path, lib_fcpeg_file_map) {
            Ok(v) => v,
            Err(e) => return Err(ChescCommandError::FCPEGError { err: e }),
        };

        match cmp.compile_into_bytecode(self.input.clone()) {
            Ok(v) => v,
            Err(e) => return Err(ChescCommandError::CompilerError { err: e }),
        };

        let end_time = start_time.elapsed();
        println!("Command process has finished in {} ms.", end_time.as_millis());
        return Ok(());
    }
}

fn main() {
    let proc = || {
        let mut cons = match Console::load(Some("src/root/Ches_1/rustnut/compiler/1.0.0/lib/lang/en-us.lang".to_string())) {
            Ok(v) => v,
            Err(_) => {
                println!("Failed to load console data.");
                return;
            },
        };

        match argh::from_env::<TopLevelCommand>().proc() {
            Ok(()) => (),
            Err(e) => cons.log(e.get_log(), true),
        }
    };

    spawn(proc).join().unwrap();
}
