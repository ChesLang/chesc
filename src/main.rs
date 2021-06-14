use rustnutc::cmd::*;
use rustnutc::compiler::compiler::*;

fn main() {
    let mut cmd_procs = std::collections::HashMap::<String, fn(String, std::collections::HashMap::<String, Vec<String>>)>::new();

    cmd_procs.insert("cmp".to_string(), chesc_cmp);

    run_command(cmd_procs);
}

fn chesc_cmp(subcmd_name: String, subcmd_options: std::collections::HashMap<String, Vec<String>>) {
    println!("- command data -");
    println!("{}", subcmd_name);
    println!("{:?}", subcmd_options);

    let input_paths = match subcmd_options.get("-i") {
        Some(v) => v,
        None => {
            println!("no input file");
            return;
        }
    };

    if input_paths.len() == 0 {
        println!("no input file");
        return;
    }

    if input_paths.len() > 1 {
        println!("too many files");
        return;
    }

    let ref_dir_paths: Vec<String> = match subcmd_options.get("-ref") {
        Some(v) => v.clone(),
        None => vec![],
    };

    let mut cmp = Compiler::new(input_paths.get(0).unwrap().clone(), ref_dir_paths, CompilerOutputKind::Executable);

    match cmp.load_src_files() {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    };
}
