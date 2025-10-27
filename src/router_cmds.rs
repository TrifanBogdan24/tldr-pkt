use colored::Colorize;

use crate::pprint::*;

pub fn router_ios_hierarchy() {
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




pub fn add_default_gateway() {
    pprint("Add default gateway", "Router(config)#", 
        "ip route 0.0.0.0 0.0.0.0 <NEXT HOP>"
    );

    println!("Example (add default gateway via 192.168.10.2):");
    pprint_cli_line("Router(config)#" ,"ip route 0.0.0.0 0.0.0.0 192.168.10.2");

}

pub fn add_static_route() {
    pprint(
        "Add static route",
        "Router(config)#",
        "ip route <DEST NETWORK> <DEST MASK> <NEXT HOP>");

    println!("Example (add route to 10.10.10.0/24 network via 192.168.10.2):");
    pprint_cli_line("Router(config)#" ,"ip route 10.10.10.0 255.255.255.0 192.168.10.2");
    println!();

}

/// Router-on-a-Stick
pub fn roas() {
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
pub fn gre() {
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



pub fn save_config() {
    pprint_cli_line("Router#", "write memory");
    println!("\
        Building configuration...\n\
        [OK]\
    ");
}


pub fn router() {
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

