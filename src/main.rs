use crate::fileman::*;

use argh::*;

use rustnutlib::*;
use rustnutlib::console::*;

use rustnutc::compiler::*;

type ChescCommandResult = Result<(), ChescCommandError>;

pub enum ChescCommandError {
    Unknown {},
    CompilerError { err: CompilerError },
    FileManError { err: FileManError },
}

impl ConsoleLogger for ChescCommandError {
    fn get_log(&self) -> ConsoleLog {
        return match self {
            ChescCommandError::Unknown {} => log!(Error, "unknown"),
            ChescCommandError::CompilerError { err } => err.get_log(),
            ChescCommandError::FileManError { err } => err.get_log(),
        };
    }
}

trait SubCommandProcessor {
    fn proc(&self) -> ChescCommandResult;
}

/// chesc command
#[derive(FromArgs, PartialEq)]
struct TopLevelCommand {
    #[argh(subcommand)]
    subcommand: SubLevelCommand,
}

impl SubCommandProcessor for TopLevelCommand {
    fn proc(&self) -> ChescCommandResult {
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
    fn proc(&self) -> ChescCommandResult {
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
    #[argh(option, short = 'r')]
    refer: Option<String>,
}

impl SubCommandProcessor for CmpSubcommand {
    fn proc(&self) -> ChescCommandResult {
        let start_time = std::time::Instant::now();
        let mut ref_dir_paths = match &self.refer {
            Some(v) => v.split(";").collect::<Vec<&str>>(),
            None => vec![],
        };

        // メインソースファイルを持つディレクトリを参照先に指定する
        let abs_main_src_path = match FileMan::get_parent_dir_path(&self.input) {
            Ok(v) => v.unwrap(),
            Err(e) => return Err(ChescCommandError::FileManError { err: e }),
        };

        ref_dir_paths.push(abs_main_src_path.to_str().unwrap());

        // todo: 環境変数 CHES_HOME の値を利用する
        let fcpeg_file_path = "src/root/Ches_1/rustnut/compiler/1.0.0/lib/fcpeg/syntax.fcpeg".to_string();

        let mut cmp = Compiler::new(self.input.clone(), ref_dir_paths.iter().map(|e| e.to_string()).collect::<Vec<String>>(), fcpeg_file_path, self.output.clone(), CompilerMode::CompiledChesc);

        match cmp.get_src_files() {
            Ok(v) => v,
            Err(e) => return Err(ChescCommandError::FileManError { err: e }),
        };

        match cmp.load_src_files() {
            Ok(v) => v,
            Err(e) => return Err(ChescCommandError::FileManError { err: e }),
        };

        match cmp.compile() {
            Ok(v) => v,
            Err(e) => return Err(ChescCommandError::CompilerError { err: e }),
        };

        let end_time = start_time.elapsed();
        println!("Command process has finished in {} ms.", end_time.as_millis());
        return Ok(());
    }
}

fn main() {
    let mut cons = match Console::load(None) {
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
}
