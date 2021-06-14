use rustnutc::cmd::*;
use rustnutc::compiler::compiler::*;
use rustnutc::console::*;

fn main() {
    let mut cmd_procs = std::collections::HashMap::<String, fn(String, std::collections::HashMap::<String, Vec<String>>, &mut Console)>::new();

    cmd_procs.insert("cmp".to_string(), chesc_cmp);

    run_command(cmd_procs);
}

fn chesc_cmp(subcmd_name: String, subcmd_options: std::collections::HashMap<String, Vec<String>>, cons: &mut Console) {
    println!("- command data -");
    println!("{}", subcmd_name);
    println!("{:?}", subcmd_options);

    let show_details = subcmd_options.contains_key("-det");

    let input_paths = match subcmd_options.get("-i") {
        Some(v) => v,
        None => {
            cons.log(ConsoleLogData::new(ConsoleLogKind::Error, "{^chesc.err.6805}", vec![], vec![format!("{{^chesc.usage}}: {{^chesc.usage.specify_input_file}}")]), show_details);
            panic!("{}", Console::get_terminate_msg());
        }
    };

    if input_paths.len() == 0 {
        cons.log(ConsoleLogData::new(ConsoleLogKind::Error, "{^chesc.err.6805}", vec![], vec![format!("{{^chesc.usage}}: {{^chesc.usage.specify_input_file}}")]), show_details);
        panic!("{}", Console::get_terminate_msg());
    }

    if input_paths.len() > 1 {
        cons.log(ConsoleLogData::new(ConsoleLogKind::Error, "{^chesc.err.6805}", vec![], vec![format!("{{^chesc.help}}: {{^chesc.help.cannot_specify_multiple_files}}")]), show_details);
        panic!("{}", Console::get_terminate_msg());
    }

    let ref_dir_paths: Vec<String> = match subcmd_options.get("-ref") {
        Some(v) => v.clone(),
        None => vec![],
    };

    let mut cmp = Compiler::new(input_paths.get(0).unwrap().clone(), ref_dir_paths, CompilerOutputKind::Executable);

    match cmp.load_src_files() {
        Ok(()) => (),
        Err(e) => {
            cons.log(e.get_log_data(), show_details);
            panic!("{}", Console::get_terminate_msg());
        }
    };
}
