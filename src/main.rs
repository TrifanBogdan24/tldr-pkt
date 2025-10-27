use colored::Colorize;
use std::env;
use std::process::exit;


pub mod pprint;
pub mod router_cmds;
pub mod switch_cmds;



use crate::pprint::*;
use crate::router_cmds::*;
use crate::switch_cmds::*;


fn add_motd_banner() {
    pprint_comment("Add MOTD (Message Of The Day) banner: will appear after equipement boots");
    pprint_cli_line("Router(config)#", "banner motd # ... your text goes here ... #");
    println!("Don't forget to save the config:");
    pprint_cli_line("Router#", "write");

    println!("Examples:");
    pprint_cli_line("Router(config)", "banner motd # ACCESS LIMITED TO AUTHORIZED PERSONNEL ONLY #");
    pprint_cli_line("Router(config)", "banner motd #\n\
        WARNING!\n\
        ACCESS IS LIMITED ONLY TO AUTHORIZED PERSONNEL!\n\
        KEEP OUT!\n\
        #");
    println!("\nResult:\n");
    println!("\
        Router> exit\n\n\
        Press RETURN to get started.\n\n\
        WARNING!\n\
        ACCESS IS LIMITED ONLY TO AUTHORIZED PERSONNEL!\n\
        KEEP OUT!\n\n\
        Router>\n");
}


fn remove_motd_banner() {
    pprint_comment("Remove MOTD (Message Of The Day) banner: will no longer appear after equipement boots");
    pprint_cli_line("Router(config)#", "no banner motd");
    println!("Don't forget to save the config:");
    pprint_cli_line("Router#", "write");
}



fn add_exec_banner() {
    pprint_comment("Add EXEC banner: will appear after entering privileged (EXEC) mode");

    println!("Examples:");
    pprint_cli_line("Router(config)", "banner exec #\n\
        WARNING!\n\
        PROCEED WITH CATION!\n\
        ALL ACTIVITY IS LOGGED AND REPORTED!\n\
        #");
    pprint_cli_line("Router(config)", "banner exec # PROCEED WITH CAUTION! #");
    println!(
        "Result:\n\
        Router> en\n\
        PROCEED WITH CAUTION!\n\
        Router#");
}

fn remove_exec_banner() {
    pprint_comment("Remove EXEC banner: will no longer appear after entering privileged (EXEC) mode");
    pprint_cli_line("Router(config)#", "no banner exec");
    println!("Don't forget to save the config:");
    pprint_cli_line("Router#", "write");
}




fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "--router" {
        router();
        exit(0);
    }



    pprint(
        "Enter privileged mode",
        "Router>",
        "[en|enable]");
    println!();


    pprint(
        "Configure terminal",
        "Router#",
        "[conf|config] [t|terminal]");
    println!();

    add_motd_banner();
    println!();
    remove_motd_banner();
    println!();
    add_exec_banner();
    println!();
    remove_exec_banner();
    println!();


    pprint("Disable DNS resolution",
            "Device(config)# ",
        "no ip domain-lookup");
    println!();


    router();
    switch();
}
