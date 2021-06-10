fn main() {
    let mut cmd_procs = std::collections::HashMap::<String, fn(String, std::collections::HashMap::<String, Vec<String>>)>::new();

    cmd_procs.insert("cmp".to_string(), chesc_cmp);

    rustnutc::cmd::run_command(cmd_procs);
}

fn chesc_cmp(subcmd_name: String, subcmd_options: std::collections::HashMap<String, Vec<String>>) {
    println!("- command data -");
    println!("{}", subcmd_name);
    println!("{:?}", subcmd_options);
}
