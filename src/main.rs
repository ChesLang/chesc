use std::cell::RefCell;
use std::collections::*;
use std::rc::Rc;
use std::thread::spawn;
use std::time::Instant;

use argh::*;

use cons_util::cons::*;
use cons_util::file::*;

use rustnutc::cmp::*;

fn main() {
    let cmd: TopLevelCommand = argh::from_env();
    let cons = Console::new("ja".to_string(), ConsoleLogLimit::NoLimit);

    match cmd.subcmd {
        Subcommand::Compile(subcmd) => spawn(move || subcmd.proc(cons)).join().unwrap(),
    }
}

trait SubCommandProcessor {
    fn proc(&self, cons: Console);
}

/// cmp subcommand
#[derive(Clone, FromArgs, PartialEq)]
#[argh(subcommand, name = "cmp")]
struct CompileSubcommand {
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

impl CompileSubcommand {
    fn compile(&self, cons: Rc<RefCell<Console>>) -> ConsoleResult<()> {
        let start_time = Instant::now();

        let mut lib_dir_paths = match &self.lib {
            Some(v) => v.split(";").collect::<Vec<&str>>(),
            None => vec![],
        };

        // メインソースファイルを持つディレクトリを参照先に指定する
        let abs_main_src_path = match FileMan::parent_dir(&self.input) {
            Ok(v) => v.unwrap(),
            Err(e) => {
                cons.borrow_mut().append_log(e.get_log());
                return Err(());
            },
        };

        lib_dir_paths.push(abs_main_src_path.to_str().unwrap());

        // todo: 環境変数 CHES_HOME の値を利用する
        let fcpeg_file_path = "src/root/Ches_1/rustnut/compiler/1.0.0/lib/fcpeg/syntax.fcpeg".to_string();

        let mut cmp = Compiler::load(cons.clone(), fcpeg_file_path, HashMap::new())?;
        cmp.compile(self.input.clone(), CompilerMode::Executable)?;

        let end_time = start_time.elapsed();
        println!("Command process has finished in {} ms.", end_time.as_millis());
        return Ok(());
    }
}

impl SubCommandProcessor for CompileSubcommand {
    fn proc(&self, cons: Console) {
        let cons_ptr = Rc::new(RefCell::new(cons));

        match self.compile(cons_ptr.clone()) {
            Ok(()) => (),
            Err(()) => {
                cons_ptr.borrow().print_all();
                return;
            }
        };
    }
}

/// cmp command
#[derive(Clone, FromArgs, PartialEq)]
struct TopLevelCommand {
    #[argh(subcommand)]
    subcmd: Subcommand,
}

#[derive(Clone, FromArgs, PartialEq)]
#[argh(subcommand)]
enum Subcommand {
    Compile(CompileSubcommand),
}
