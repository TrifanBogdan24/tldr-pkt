use colored::Colorize;

use crate::pprint::{
    pprint_cli_line,
    pprint_comment,
    pprint_cli_line_with_comment,
    pprint
};

pub fn switch_ios_hierarchy() {
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


pub fn add_vlan() {
    pprint_comment("Add VLAN:");
    pprint_cli_line("Switch(config)#", "vlan <ID>");
    pprint_cli_line("Switch(config-vlan)#", "exit");
    pprint_cli_line("Switch(config)#", "");
    println!("Example:");
    pprint_cli_line("Switch(config)#", "vlan 10");
    pprint_cli_line("Switch(config-vlan)#", "exit");

}


pub fn show_vlan() {
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


pub fn show_vlan_brief() {
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




pub fn config_access_port() {
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



pub fn config_trunk_port() {
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


pub fn config_multiple_trunk_ports() {
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

pub fn config_native_vlan() {
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


pub fn management_vlan_interface() {
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



pub fn remove_vlan() {
    pprint_comment("Remove a VLAN (by ID):");
    pprint_cli_line("Switch(config)#", "no vlan <ID>");
    println!("Example:");
    pprint_cli_line("Switch(config)#", "no vlan 10");
}



pub fn switch() {
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