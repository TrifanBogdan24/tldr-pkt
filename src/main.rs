use colored::Colorize;


fn pretty_print_cli_line(prompt: &str, command: &str) {
    println!("{} {}",
        prompt.yellow(),
        command.yellow().bold());

}


fn pretty_print_comment(comment: &str) {
    println!("{}",
        comment.cyan().bold()
    );
}



fn pretty_print(comment: &str, prompt: &str, command: &str) {
    println!("{}\n{} {}",
        comment.cyan().bold(),
        prompt.yellow(),
        command.yellow().bold());
}


fn print_info_mask_format() {
    println!("Network's MASK must be specified in decimal notation, not prefix.\n\
        Please visit: https://www.calculator.net/ip-subnet-calculator.html")
}

fn add_default_gateway() {
    pretty_print("Add default gateway", "Router(config)#", 
        "ip route 0.0.0.0 0.0.0.0 <NEXT HOP>"
    );

    println!("Example (add default gateway via 192.168.10.2):");
    pretty_print_cli_line("Router(config)#" ,"ip route 0.0.0.0 0.0.0.0 192.168.10.2");

}

fn add_static_route() {
    pretty_print(
        "Add static route",
        "Router(config)#",
        "ip route <DEST NETWORK> <DEST MASK> <NEXT HOP>");

    println!("Example (add route to 10.10.10.0/24 network via 192.168.10.2):");
    pretty_print_cli_line("Router(config)#" ,"ip route 10.10.10.0 255.255.255.0 192.168.10.2");
    println!();

}


fn router_on_a_stick() {
    println!(
        "{} {} {}",
        "Configure".cyan().bold(),
        "RoaS".green().bold(),
        "(Router on a Stick)".cyan().bold()
    );

    pretty_print_cli_line("Router>", "enable");
    pretty_print_cli_line("Router#", "configure terminal");
    pretty_print_cli_line("Router(config)#" ,"interface <intf name>.<sub interface ID>");
    pretty_print_cli_line("Router(config-subif)#", "encapsulation dot1Q <VLAN ID>");
    pretty_print_cli_line("Router(config-subif)#", "ip address <IP> <MASK>");
    pretty_print_cli_line("Router(config-subif)#", "exit");
    pretty_print_cli_line("Router(config)", "interface <intf name>.<sub interface ID>");
    pretty_print_cli_line("Router(config-subif)#", "encapsulation dot1Q <VLAN ID>");
    pretty_print_cli_line("Router(config-subif)#", "ip address <IP> <MASK>");
    pretty_print_cli_line("Router(config-subif)#","exit");
    pretty_print_cli_line("Router(config)#", "interface <intf name>");
    pretty_print_cli_line("Router(config-if)#", "no shutdown");
    pretty_print_cli_line("Router(config-if)#", "exit");
    pretty_print_cli_line("Router(config)#", "exit");
    pretty_print_cli_line("Router#", "write");
    pretty_print_cli_line("Router#", "exit");

    println!("Example {} (Router on a Stick):", "RoaS".green().bold());
    pretty_print_cli_line("Router>", "enable");
    pretty_print_cli_line("Router#", "configure terminal");
    pretty_print_cli_line("Router(config)#" ,"interface gigabitEthernet 0/0/0.1");
    pretty_print_cli_line("Router(config-subif)#", "encapsulation dot1Q 10");
    pretty_print_cli_line("Router(config-subif)#", "ip address 10.10.10.1 255.255.255.0");
    pretty_print_cli_line("Router(config-subif)#", "exit");
    pretty_print_cli_line("Router(config)", "interface gigabitEthernet 0/0/0.20");
    pretty_print_cli_line("Router(config-subif)#", "encapsulation dot1Q 2");
    pretty_print_cli_line("Router(config-subif)#", "ip address 20.20.20.1 255.255.255.0");
    pretty_print_cli_line("Router(config-subif)#","exit");
    pretty_print_cli_line("Router(config)#", "interface gigabitEthernet 0/0/0");
    pretty_print_cli_line("Router(config-if)#", "no shutdown");
    pretty_print_cli_line("Router(config-if)#", "exit");
    pretty_print_cli_line("Router(config)#", "exit");
    pretty_print_cli_line("Router#", "write memory");
    pretty_print_cli_line("Router#", "exit");



}


fn router() {
    pretty_print("Display router's interfaces", 
        "Router#",
        "[sh|show] ip [int|inter|interface] [br|brief]");

    println!(
        "Interface              IP-Address      OK? Method Status                Protocol \n\
        FastEthernet0/0        10.23.64.1      YES manual up                    up \n\
        FastEthernet1/0        10.24.64.2      YES manual up                    up \n\
        Serial2/0              unassigned      YES unset  administratively down down \n\
        Serial3/0              unassigned      YES unset  administratively down down \n\
        FastEthernet4/0        unassigned      YES unset  administratively down down \n\
        FastEthernet5/0        unassigned      YES unset  administratively down down \n\
        Modem6/0               unassigned      YES unset  administratively down down \n\
        Modem7/0               unassigned      YES unset  administratively down down \n\
        Modem8/0               unassigned      YES unset  administratively down down \n\
        Modem9/0               unassigned      YES unset  administratively down down \n\
    ");


    pretty_print_comment("Enter router's interface");
    pretty_print_cli_line("Router(config)#", "[int|inter|interface] <interface name>");
    println!("Example:");
    pretty_print_cli_line("Router(config)#", "int FastEthernet1/0");
    pretty_print_cli_line("Router(config)#", "int fa1/0");
    pretty_print_cli_line("Router(config)#", "int GigabitEthernet0/1");
    pretty_print_cli_line("Router(config)#", "int gig0/1");
    println!();

    

    pretty_print(
        "Assign IPv4 address",
        "Router(config-if)#",
        "ip [add|addr|address] <IP> <MASK>"
    );

    println!("Example (place 192.168.14.1/24):");
    pretty_print_cli_line(
        "Router(config-if)#",
        "ip address 192.168.14.1 255.255.255.0");
    println!();
    print_info_mask_format();

    println!();

    pretty_print(
        "Flush (remove) IP address", 
        "Router(config-if)# ",
        "no ip [add|addr|address]"
    );
    println!();

    add_static_route();
    add_default_gateway();
    println!();

    router_on_a_stick();
}


fn switch() {

}



fn add_motd_banner() {
    pretty_print_comment("Add MOTD (Message Of The Day) banner: will appear after equipement boots");
    pretty_print_cli_line("Router(config)#", "banner motd # ... your text goes here ... #");
    println!("Don't forget to save the config:");
    pretty_print_cli_line("Router#", "write");

    println!("Examples:");
    pretty_print_cli_line("Router(config)", "banner motd # ACCESS LIMITED TO AUTHORIZED PERSONNEL ONLY #");
    pretty_print_cli_line("Router(config)", "banner motd #\n\
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
    pretty_print_comment("Remove MOTD (Message Of The Day) banner: will no longer appear after equipement boots");
    pretty_print_cli_line("Router(config)#", "no banner motd");
    println!("Don't forget to save the config:");
    pretty_print_cli_line("Router#", "write");
}



fn add_exec_banner() {
    pretty_print_comment("Add EXEC banner: will appear after entering privileged (EXEC) mode");

    println!("Examples:");
    pretty_print_cli_line("Router(config)", "banner exec #\n\
        WARNING!\n\
        PROCEED WITH CATION!\n\
        ALL ACTIVITY IS LOGGED AND REPORTED!\n\
        #");
    pretty_print_cli_line("Router(config)", "banner exec # PROCEED WITH CAUTION! #");
    println!(
        "Result:\n\
        Router> en\n\
        PROCEED WITH CAUTION!\n\
        Router#");
}

fn remove_exec_banner() {
    pretty_print_comment("Remove EXEC banner: will no longer appear after entering privileged (EXEC) mode");
    pretty_print_cli_line("Router(config)#", "no banner exec");
    println!("Don't forget to save the config:");
    pretty_print_cli_line("Router#", "write");
}


fn main() {
    pretty_print(
        "Enter privileged mode",
        "Router>",
        "[en|enable]");
    println!();


    pretty_print(
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


    pretty_print("Disable DNS resolution",
            "Device(config)# ",
        "no ip domain-lookup");
    println!();


    router();
    switch();
}
