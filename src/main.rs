use colored::Colorize;
use std::env;
use std::process::exit;

fn pprint_cli_line(prompt: &str, command: &str) {
    println!("{} {}",
        prompt.yellow(),
        command.yellow().bold());
}


fn pprint_cli_line_with_comment(prompt: &str, command: &str, comment: &str) {
    println!("{} {} {}",
        prompt.yellow(),
        command.yellow().bold(),
        comment
    );
}

fn pprint_comment(comment: &str) {
    println!("{}",
        comment.cyan().bold()
    );
}



fn pprint(comment: &str, prompt: &str, command: &str) {
    println!("{}\n{} {}",
        comment.cyan().bold(),
        prompt.yellow(),
        command.yellow().bold());
}


fn print_info_mask_format() {
    println!("Network's MASK must be specified in decimal notation, not prefix.\n\
        Please visit: https://www.calculator.net/ip-subnet-calculator.html")
}



fn router_ios_hierarchy() {
    println!("\
        IOS Router Hierarchy:\n\n\
        +---------------------------+\n\
        |        User EXEC          |\n\
        +---------------------------+\n\
        |     Privileged EXEC       |\n\
        +---------------------------+\n\
        |      Global Config        |\n\
        +-----------+------+--------+\n\
        | Interface | Line | Router |\n\
        +-----------+------+--------+\n\
    ");

    pprint_comment("User EXEC mode (default mode after boot):");
    pprint_cli_line("Router>", "");

    pprint_comment("Enter Privileged EXEC mode (access to show and tests commands):");
    pprint_cli_line("Router>", "enable");
    pprint_cli_line("Router#", "");

    pprint_comment("Enter Global Config mode:");
    pprint_cli_line("Router#", "configure terminal");
    pprint_cli_line("Router(config)#", "");

    pprint_comment("Configure an interface:");
    pprint_cli_line("Router(config)#", "interface <interf-name> <interf-number>");
    pprint_cli_line("Router(config-if)#", "");


    pprint_comment("Configure a console line:");
    pprint_cli_line("Router(config)#", "line <line-name> <line-number>");
    pprint_cli_line("Router(config-line)#", "");

    pprint_comment("Configure a routing protocol:");
    pprint_cli_line("Router(config)#", "router <routing-protocol-name>");
    pprint_cli_line("Router(config-router)#", "");
    println!();
}




fn add_default_gateway() {
    pprint("Add default gateway", "Router(config)#", 
        "ip route 0.0.0.0 0.0.0.0 <NEXT HOP>"
    );

    println!("Example (add default gateway via 192.168.10.2):");
    pprint_cli_line("Router(config)#" ,"ip route 0.0.0.0 0.0.0.0 192.168.10.2");

}

fn add_static_route() {
    pprint(
        "Add static route",
        "Router(config)#",
        "ip route <DEST NETWORK> <DEST MASK> <NEXT HOP>");

    println!("Example (add route to 10.10.10.0/24 network via 192.168.10.2):");
    pprint_cli_line("Router(config)#" ,"ip route 10.10.10.0 255.255.255.0 192.168.10.2");
    println!();

}

/// Router-on-a-Stick
fn roas() {
    println!(
        "{} {} {}",
        "Configure".cyan().bold(),
        "RoaS".green().bold(),
        "(Router on a Stick)".cyan().bold()
    );

    pprint_cli_line("Router>", "enable");
    pprint_cli_line("Router#", "configure terminal");
    pprint_cli_line("Router(config)#" ,"interface <intf name>.<sub interface ID>");
    pprint_cli_line("Router(config-subif)#", "encapsulation dot1Q <VLAN ID>");
    pprint_cli_line("Router(config-subif)#", "ip address <IP> <MASK>");
    pprint_cli_line("Router(config-subif)#", "exit");
    pprint_cli_line("Router(config)", "interface <intf name>.<sub interface ID>");
    pprint_cli_line("Router(config-subif)#", "encapsulation dot1Q <VLAN ID>");
    pprint_cli_line("Router(config-subif)#", "ip address <IP> <MASK>");
    pprint_cli_line("Router(config-subif)#","exit");
    pprint_cli_line("Router(config)#", "interface <intf name>");
    pprint_cli_line("Router(config-if)#", "no shutdown");
    pprint_cli_line("Router(config-if)#", "exit");
    pprint_cli_line("Router(config)#", "exit");
    pprint_cli_line("Router#", "write");
    pprint_cli_line("Router#", "exit");

    println!("Example {} (Router on a Stick):", "RoaS".green().bold());
    pprint_cli_line("Router>", "enable");
    pprint_cli_line("Router#", "configure terminal");
    pprint_cli_line("Router(config)#" ,"interface gigabitEthernet 0/0/0.1");
    pprint_cli_line("Router(config-subif)#", "encapsulation dot1Q 10");
    pprint_cli_line("Router(config-subif)#", "ip address 10.10.10.1 255.255.255.0");
    pprint_cli_line("Router(config-subif)#", "exit");
    pprint_cli_line("Router(config)", "interface gigabitEthernet 0/0/0.20");
    pprint_cli_line("Router(config-subif)#", "encapsulation dot1Q 2");
    pprint_cli_line("Router(config-subif)#", "ip address 20.20.20.1 255.255.255.0");
    pprint_cli_line("Router(config-subif)#","exit");
    pprint_cli_line("Router(config)#", "interface gigabitEthernet 0/0/0");
    pprint_cli_line("Router(config-if)#", "no shutdown");
    pprint_cli_line("Router(config-if)#", "exit");
    pprint_cli_line("Router(config)#", "exit");
    pprint_cli_line("Router#", "write memory");
    pprint_cli_line("Router#", "exit");



}



/// Generic Routing Encapsulation
fn gre() {
    println!("{} {} {}",
        "Configure".cyan().bold(),
        "GRE".green().bold(),
        "(Generic Routing Encapsulation):".cyan().bold());
    println!("On Router 0:");
    println!("{} {}",
        "STEP 1:".cyan().bold(), "Create virtual tunnel interface:");
    pprint_cli_line("R0(config)", "[int|interface] tunnel <NR>");
    println!("{} {}",
        "STEP 2:".cyan().bold(),
        "Assign tunnel IP and subnet:");
    pprint_cli_line("R0(config-if)", "ip [a|addr|address] <TUNNEL_IP1> <TUNNEL_MASK>");
    println!("{} {}",
        "STEP 3:".cyan().bold(),
        "Use local physical interface as tunnel source:");
    pprint_cli_line("R0(config-if)", "tunnel source <INTERFACE_NAME>");
    println!("{} {}",
        "STEP 4:".cyan().bold(),
        "Define remote router’s IP (physical interface) as tunnel destination:");
    pprint_cli_line("R0(config-if)", "tunnel destination <IP of R1>");
   
    println!("On Router 1:");
    println!("{} {}",
        "STEP 1:".cyan().bold(),
        "Create virtual tunnel interface:");
    pprint_cli_line("R1(config)", "[int|interface] tunnel <NR>");
    println!("{} {}",
        "STEP 2:".cyan().bold(),
        "Assign tunnel IP and subnet:");
    pprint_cli_line("R1(config-if)", "ip [a|addr|address] <TUNNEL_IP2> <TUNNEL_MASK>");
    println!("{} {}",
        "STEP 3:".cyan().bold(),
        "Use local physical interface as tunnel source:");
    pprint_cli_line("R1(config-if)", "tunnel source <INTERFACE_NAME>");
    println!("{} {}",
        "STEP 4:".cyan().bold(),
        "Define remote router’s IP (physical interface) as tunnel destination:");
    pprint_cli_line("R1(config-if)", "tunnel destination <IP of R0>");
    println!();
   
    println!("Example:");
    println!("On Router 0:");
    pprint_cli_line_with_comment("R0(config)", "interface tunnel 1", "                    ! Create GRE tunnel");
    pprint_cli_line_with_comment("R0(config-if)", "ip address 50.50.50.1 255.255.255.0", "! Assign 1st IP on tunnel endpoint");
    pprint_cli_line_with_comment("R0(config-if)", "tunnel source FastEthernet 0/0", "     ! Local interface");
    pprint_cli_line_with_comment("R0(config-if)", "tunnel destination 20.0.0.1", "        ! Remote (physical) IP");
    println!("On Router 1:");
    pprint_cli_line_with_comment("R1(config)", "interface tunnel 1", "                    ! Create GRE tunnel");
    pprint_cli_line_with_comment("R1(config-if)", "ip address 50.50.50.2 255.255.255.0", "! Assign 2nd IP on tunnel endpoint");
    pprint_cli_line_with_comment("R1(config-if)", "tunnel source FastEthernet 0/0", "     ! Local interface");
    pprint_cli_line_with_comment("R1(config-if)", "tunnel destination 10.0.0.1", "        ! Remote (physical) IP");
    println!("Learn more here: https://ipcisco.com/lesson/gre-tunnel-configuration-with-cisco-packet-tracer/");
    println!();
    println!("\
        NOTE: Note all Packet Tracer routers support GRE\n\
        - Cisco 1941
        - Cisco 2901
        - Cisco 2911
        - Cisco 4321 (if available)
        ");
}



fn save_config() {
    pprint_cli_line("Router#", "write memory");
    println!("\
        Building configuration...\n\
        [OK]\
    ");
}


fn router() {
    router_ios_hierarchy();

    pprint("Display router's interfaces", 
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


    pprint_comment("Enter router's interface");
    pprint_cli_line("Router(config)#", "[int|inter|interface] <interface name>");
    println!("Example:");
    pprint_cli_line("Router(config)#", "int FastEthernet1/0");
    pprint_cli_line("Router(config)#", "int fa1/0");
    pprint_cli_line("Router(config)#", "int GigabitEthernet0/1");
    pprint_cli_line("Router(config)#", "int gig0/1");
    println!();

    

    pprint(
        "Assign IPv4 address",
        "Router(config-if)#",
        "ip [add|addr|address] <IP> <MASK>"
    );

    println!("Example (place 192.168.14.1/24):");
    pprint_cli_line(
        "Router(config-if)#",
        "ip address 192.168.14.1 255.255.255.0");
    println!();
    print_info_mask_format();

    println!();

    pprint(
        "Flush (remove) IP address", 
        "Router(config-if)# ",
        "no ip [add|addr|address]"
    );
    println!();

    add_static_route();
    add_default_gateway();
    println!();

    roas();  // Router-on-a-Stick
    gre();   // Generic Routing Encapsulation
}



fn switch_ios_hierarchy() {
    println!("\
        IOS Switch Hierarchy:\n\n\
        +---------------------------+\n\
        |        User EXEC          |\n\
        +---------------------------+\n\
        |     Privileged EXEC       |\n\
        +---------------------------+\n\
        |      Global Config        |\n\
        +-----------+------+--------+\n\
        | Interface | Line |  VLAN  |\n\
        +-----------+------+--------+\n\
    ");


    pprint_comment("User EXEC mode (default mode after boot):");
    pprint_cli_line("Switch>", "");

    pprint_comment("Enter Privileged EXEC mode (access to show and tests commands):");
    pprint_cli_line("Switch>", "enable");
    pprint_cli_line("Switch#", "");

    pprint_comment("Enter Global Config mode:");
    pprint_cli_line("Switch#", "configure terminal");
    pprint_cli_line("Switch(config)#", "");

    pprint_comment("Configure an interface:");
    pprint_cli_line("Switch(config)#", "interface <interf-name> <interf-number>");
    pprint_cli_line("Switch(config-if)#", "");


    pprint_comment("Configure a console line:");
    pprint_cli_line("Switch(config)#", "line <line-name> <line-number>");
    pprint_cli_line("Switch(config-line)#", "");

    pprint_comment("Configure a VLAN:");
    pprint_cli_line("Switch(config)#", "vlan <vlan-number>");
    pprint_cli_line("Switch(config-vlan)#", "");

    pprint_comment("Create management VLAN (assing IP on a virtual interface):");
    pprint_cli_line("Switch(config)#", "interface vlan <vlan-number>");
    pprint_cli_line("Switch(config-if)#", "ip address <ip-address> <subnet-mask>");
    pprint_cli_line("Switch(config-if)#", "no shutdown");
}


fn add_vlan() {
    pprint_comment("Add VLAN:");
    pprint_cli_line("Switch(config)#", "vlan <ID>");
    pprint_cli_line("Switch(config-vlan)#", "exit");
    pprint_cli_line("Switch(config)#", "");
    println!("Example:");
    pprint_cli_line("Switch(config)#", "vlan 10");
    pprint_cli_line("Switch(config-vlan)#", "exit");

}


fn show_vlan() {
    pprint("", "Switch#", "[sh|show] vlan");
    println!("\
        VLAN Name                             Status    Ports \n\
        ---- -------------------------------- --------- -------------------------------\n
        1    default                          active    Fa0/1, Fa1/1, Fa2/1, Fa3/1\n\
                                                        Fa4/1, Fa5/1\n\
        10   VLAN0010                         active    \n\
        1002 fddi-default                     active    \n\
        1003 token-ring-default               active    \n\
        1004 fddinet-default                  active    \n\
        1005 trnet-default                    active    \n\
        \n\
        VLAN Type  SAID       MTU   Parent RingNo BridgeNo Stp  BrdgMode Trans1 Trans2\n\
        ---- ----- ---------- ----- ------ ------ -------- ---- -------- ------ ------\n\
        1    enet  100001     1500  -      -      -        -    -        0      0\n\
        10   enet  100010     1500  -      -      -        -    -        0      0\n\
        1002 fddi  101002     1500  -      -      -        -    -        0      0\n\
        1003 tr    101003     1500  -      -      -        -    -        0      0\n\
        1004 fdnet 101004     1500  -      -      -        ieee -        0      0\n\
        1005 trnet 101005     1500  -      -      -        ibm  -        0      0\n\
        \n\
        VLAN Type  SAID       MTU   Parent RingNo BridgeNo Stp  BrdgMode Trans1 Trans2\n\
        ---- ----- ---------- ----- ------ ------ -------- ---- -------- ------ ------\n\
        \n\
        Remote SPAN VLANs\n\
        ------------------------------------------------------------------------------\n\
        \n\
        Primary Secondary Type              Ports\n\
        ------- --------- ----------------- ------------------------------------------\n\
    ");



}


fn show_vlan_brief() {
    pprint("", "Switch#", "[sh|show] vlan [br|brief]");
    println!("\
        VLAN Name                             Status    Ports\n\
        ---- -------------------------------- --------- -------------------------------\n\
        1    default                          active    Fa0/1, Fa1/1, Fa2/1, Fa3/1\n\
                                                        Fa4/1, Fa5/1\n\
        10   VLAN0010                         active    \n\
        1002 fddi-default                     active    \n\
        1003 token-ring-default               active    \n\
        1004 fddinet-default                  active    \n\
        1005 trnet-default                    active \n\
    ");
}




fn config_access_port() {
    pprint_cli_line("Switch(config)#", "Switch(config)# [int|inter|interface] <interface name>");
    pprint_comment("Set port type (access/trunk):");
    pprint_cli_line("Switch(config-if)#", "switchport mode access");
    pprint_comment("Set VLAN ID:");
    pprint_cli_line("Switch(config-if)#", "switchport access vlan <VLAN-ID>");
    println!("Example:");
    pprint_cli_line("Switch(config-if)#", "switchport access vlan 10");
    println!();
    println!("\
        NOTE: Without the first command `switchport mode access`, the port will stay in dynamic desirable mode, meaning:\n\
        - It may try to negociate a trunk port with a connected switch\n\
        - and it may become a trunk port if the other device accepts\n\
        \n\
        As result, the config is unpredictable and not purely \"access\" port\n\
    ")
}



fn config_trunk_port() {
    pprint_cli_line("Switch#", "[sh|show] ip [int|interface] [br|brief]");
    println!("\
        Interface              IP-Address      OK? Method Status                Protocol\n\
        FastEthernet0/1        unassigned      YES manual down                  down \n\
        FastEthernet1/1        unassigned      YES manual down                  down \n\
        FastEthernet2/1        unassigned      YES manual down                  down \n\
        FastEthernet3/1        unassigned      YES manual down                  down \n\
        Vlan1                  unassigned      YES manual administratively down down\n\
    ");
    pprint_cli_line("Switch#", "[conf|config] [t|term|terminal]");
    pprint_cli_line("Switch(config)#", "Switch(config)# [int|inter|interface] <interface name>");
    pprint_comment("Set port type (access/trunk):");
    pprint_cli_line("Switch(config-if-range)#", "switchport mode trunk");
    println!("NOTE: trunk VLAN must be configured at both ends of a link");
}


fn config_multiple_trunk_ports() {
    pprint_cli_line("Switch#", "[sh|show] ip [int|interface] [br|brief]");
    println!("\
        Interface              IP-Address      OK? Method Status                Protocol\n\
        FastEthernet0/1        unassigned      YES manual down                  down \n\
        FastEthernet1/1        unassigned      YES manual down                  down \n\
        FastEthernet2/1        unassigned      YES manual down                  down \n\
        FastEthernet3/1        unassigned      YES manual down                  down \n\
        Vlan1                  unassigned      YES manual administratively down down\n\
    ");
    pprint_cli_line("Switch#", "[conf|config] [t|term|terminal]");
    pprint_cli_line("Switch(config)#", "Switch(config)# interface range fa0/1 - fa4/1");
    pprint_comment("Set port type (access/trunk):");
    pprint_cli_line("Switch(config-if-range)#", "switchport mode trunk");
    println!("NOTE: trunk VLAN must be configured at both ends of a link");
}

fn config_native_vlan() {
    pprint_comment("Configure native VLAN:");
    pprint_cli_line("Switch(config)#", "[int|interface] <interface name>");
    pprint_cli_line("Switch(config-if)#", "switchport trunk native vlan <ID>");
    println!("Example:");
    pprint_cli_line("Switch(config)#", "[int|interface] FastEthernet 0/1");
    pprint_cli_line("Switch(config-if)#", "switchport trunk native vlan 30");

    println!("NOTE:\n\
        - A native VLAN is a placed on trunk line, so that line will remove 802.1Q of the specified VLAN\n\
        - Native VLANs are deprecated (security vulnerability: double tagging; place a native VLAN that is not in VLAN's database)\n\
        - The native (trunk) VLAN must be configured at both ends of a link\
    ")
}


fn management_vlan_interface() {
    pprint_comment("STEP 1: Create a new VLAN, for management:");
    pprint_cli_line("Switch(config)#", "vlan <ID>");
    pprint_cli_line("Switch(config-vlan)#", "exit");
    pprint_cli_line("Switch(config)#", "");
    pprint_comment("STEP 2: Assign IP address on management VLAN interface:");
    pprint_cli_line("Switch(config)#", "interface vlan <ID>");
    pprint_cli_line("Switch(config-if)#", "ip address <IP> <MASK>");
    println!();
    println!("Example:");
    pprint_cli_line("Switch(config)#", "vlan 101");
    pprint_cli_line("Switch(config-vlan)#", "exit");
    pprint_cli_line("Switch(config)#", "interface vlan 101");
    pprint_cli_line("Switch(config-if)#", "ip address 10.10.10.99 255.255.255.0");

    println!();
    println!(
        "NOTE:\n\
        - A switch can only have 1 management VLAN\n\
        - The management VLAN has a virtual interface (not a physical one, like FastEthernet0/1)
    ");

}



fn remove_vlan() {
    pprint_comment("Remove a VLAN (by ID):");
    pprint_cli_line("Switch(config)#", "no vlan <ID>");
    println!("Example:");
    pprint_cli_line("Switch(config)#", "no vlan 10");
}



fn switch() {
    switch_ios_hierarchy();
    add_vlan();
    show_vlan();
    show_vlan_brief();
    config_access_port();
    config_trunk_port();
    config_multiple_trunk_ports();
    config_native_vlan();
    management_vlan_interface();
    remove_vlan();
}



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
