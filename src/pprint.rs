use colored::Colorize;

pub fn pprint_cli_line(prompt: &str, command: &str) {
    println!("{} {}",
        prompt.yellow(),
        command.yellow().bold());
}


pub fn pprint_cli_line_with_comment(prompt: &str, command: &str, comment: &str) {
    println!("{} {} {}",
        prompt.yellow(),
        command.yellow().bold(),
        comment
    );
}

pub fn pprint_comment(comment: &str) {
    println!("{}",
        comment.cyan().bold()
    );
}



pub fn print_info_mask_format() {
    println!("Network's MASK must be specified in decimal notation, not prefix.\n\
        Please visit: https://www.calculator.net/ip-subnet-calculator.html")
}

pub fn pprint(comment: &str, prompt: &str, command: &str) {
    println!("{}\n{} {}",
        comment.cyan().bold(),
        prompt.yellow(),
        command.yellow().bold());
}