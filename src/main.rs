use rustnutlib::*;
use rustnutc::compiler;

fn main() {
    let mut cmd_procs = cmd::CommandMap::new();
    cmd_procs.insert("cmp".to_string(), chesc_cmp);
    cmd::run_command("cmp", cmd_procs);
}

fn chesc_cmp(_subcmd_name: String, subcmd_options: std::collections::HashMap<String, Vec<String>>, cons: &mut console::Console) {
    // println!("- command data -");
    // println!("{}", subcmd_name);
    // println!("{:?}", subcmd_options);

    let start_time = std::time::Instant::now();

    let show_details = subcmd_options.contains_key("-det");

    let input_paths = match subcmd_options.get("-i") {
        Some(v) => v,
        None => {
            cons.log(console::ConsoleLogData::new(console::ConsoleLogKind::Error, "{^chesc.err.6805}", vec![], vec![format!("{{^chesc.usage}}: {{^chesc.usage.specify_input_file}}")]), show_details);
            return;
        }
    };

    if input_paths.len() == 0 {
        cons.log(console::ConsoleLogData::new(console::ConsoleLogKind::Error, "{^chesc.err.6805}", vec![], vec![format!("{{^chesc.usage}}: {{^chesc.usage.specify_input_file}}")]), show_details);
        return;
    }

    if input_paths.len() > 1 {
        cons.log(console::ConsoleLogData::new(console::ConsoleLogKind::Error, "{^chesc.err.6805}", vec![], vec![format!("{{^chesc.help}}: {{^chesc.help.cannot_specify_multiple_files}}")]), show_details);
        return;
    }

    let mut ref_dir_paths: Vec<String> = match subcmd_options.get("-ref") {
        Some(v) => v.clone(),
        None => vec![],
    };

    // メインソースファイルを持つディレクトリを参照先に指定する
    let abs_main_src_path = match fileman::FileMan::get_parent_dir_path(&input_paths.get(0).unwrap()) {
        Ok(v) => {
            if v.is_none() {
                cons.log(console::ConsoleLogData::new(console::ConsoleLogKind::Error, "{^chesc.err.2711}", vec![], vec![]), show_details);
            }

            v.unwrap()
        },
        Err(e) => {
            cons.log(e.get_log_data(), show_details);
            return;
        },
    };

    ref_dir_paths.push((*abs_main_src_path).to_str().unwrap().to_string());
    let fcpeg_file_path = "src/fcpeg/syntax.fcpeg".to_string();
    let output_file_path = "src/ches/test.chesc".to_string();
    let mut cmp = compiler::Compiler::new(input_paths.get(0).unwrap().clone(), ref_dir_paths, fcpeg_file_path, output_file_path, compiler::CompilerMode::CompiledChesc);

    match cmp.get_src_files() {
        Ok(()) => (),
        Err(e) => {
            cons.log(e.get_log_data(), show_details);
            return;
        }
    };

    match cmp.load_src_files() {
        Ok(()) => (),
        Err(e) => {
            cons.log(e.get_log_data(), show_details);
            return;
        }
    };

    match cmp.compile() {
        Ok(()) => (),
        Err(e) => {
            cons.log(e.get_log_data(), show_details);
            return;
        }
    };

    let end_time = start_time.elapsed();
    println!("Command process has finished in {} ms.", end_time.as_millis());
}
