// @ziggoon - rustyURLscan v1.0
// pure rust url scan + ip blocker <3

use rustyline::error::ReadlineError;
use rustyline::Editor;

// internal package
use crate::util;

fn banner() {
	let banner = r#"
██████╗░██╗░░░██╗░██████╗████████╗██╗░░░██╗██╗░░░██╗██████╗░██╗░░░░░░██████╗░█████╗░░█████╗░███╗░░██╗
██╔══██╗██║░░░██║██╔════╝╚══██╔══╝╚██╗░██╔╝██║░░░██║██╔══██╗██║░░░░░██╔════╝██╔══██╗██╔══██╗████╗░██║
██████╔╝██║░░░██║╚█████╗░░░░██║░░░░╚████╔╝░██║░░░██║██████╔╝██║░░░░░╚█████╗░██║░░╚═╝███████║██╔██╗██║
██╔══██╗██║░░░██║░╚═══██╗░░░██║░░░░░╚██╔╝░░██║░░░██║██╔══██╗██║░░░░░░╚═══██╗██║░░██╗██╔══██║██║╚████║
██║░░██║╚██████╔╝██████╔╝░░░██║░░░░░░██║░░░╚██████╔╝██║░░██║███████╗██████╔╝╚█████╔╝██║░░██║██║░╚███║
╚═╝░░╚═╝░╚═════╝░╚═════╝░░░░╚═╝░░░░░░╚═╝░░░░╚═════╝░╚═╝░░╚═╝╚══════╝╚═════╝░░╚════╝░╚═╝░░╚═╝╚═╝░░╚══╝ "#;
	println!("{}", banner);
}

fn description() {
    let desc = r#"
[+] Welcome to RustyURLScan, a tool designed to assist in blocking malicious domains for host-based firewalls.
[+] powered by urlscan.io (API key required or else queries are rate-limited hard)
[+] written by @ziggoon | v1.0 
"#;
    println!("{}", desc);
}

// readline utility from @postrequest 
pub fn cli_line(prompt: &str) -> Vec<String> {
    use std::io::{stdin, stdout, Write};
    print!("{}", prompt);
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    if s.is_empty() {
        return vec![String::from("")];
    }
    get_string_vec(s)
}

fn get_string_vec(s: String) -> Vec<String> {
    if s.is_empty() {
        return vec![String::from("")];
    }
    s.split_whitespace().map(str::to_string).collect()
}

fn main_help() {
    println!("  set-api-key       sets urlscan.io api key");
    println!("                    usage: set-api-key <key>\n");
    println!("  query             queries a domain/url using urlscan.io");
    println!("                    usage: query <domain>\n");
    println!("  help              this menu lol");
    println!("  quit              exits the program");
}

pub fn main_loop() {
    banner();
    description();

    // initialize user_input to a mutable String
    let mut user_input: Vec<String>;

    let mut rl = Editor::<()>::new();
    if rl.load_history(".history.txt").is_err() {
           println!("no previous history...");
    }    
    loop {
        let readline = rl.readline("RustyURLScan>> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                user_input = get_string_vec(line);
                match user_input[0].as_str() {
                    "set-api-key" => util::commands::set_api_key(user_input),
                    "query" => util::commands::query(),
                    "set-rule" => util::commands::set_rule(),
                    "clear-chain" => util::commands::clear_chain(),
                    "ipt-chain" => util::commands::create_iptables_chain(),
                    "help" => main_help(),
                    "quit" => std::process::exit(0),
                    _ => continue,
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("ctrl+c pressed. quitting now..");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("ctrl+d pressed. quitting now..");
                break
            },
            Err(err) => {
                println!("error: {:?}", err);
                break
            }
        } 
    }
    rl.save_history(".history.txt").unwrap();
}
