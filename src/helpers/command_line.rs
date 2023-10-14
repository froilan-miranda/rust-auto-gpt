use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();

        // Decide on the print color
        let statement_color: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };

        // Print the agent statement
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();

        print!("Agent: {}: ", agent_pos);

        // Reset Color
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        // Reset Color
        stdout.execute(ResetColor).unwrap();
    }
}

// Get user request
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    // Print the question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    // Reset color
    stdout.execute(ResetColor).unwrap();

    // Read user input
    let mut user_response: String = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read line");

    // trim whitespace and return
    return user_response.trim().to_string();
}

pub fn confirm_safe_code() -> bool {
    let mut stdout: std::io::Stdout = stdout();
    loop {
        // Print the question in specified color
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        print!("");
        print!("WARNING: You are about to run code written entirely by AI ");
        println!("Review your code and confirm you with to continue.");

        // Reset Color
        stdout.execute(ResetColor).unwrap();

        // Present options with different colors
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] All good");
        stdout.execute(SetForegroundColor(Color::Red)).unwrap();
        println!("[2] Lets stop this projet");

        // Reset Color
        stdout.execute(ResetColor).unwrap();

        // Read user input
        let mut human_input: String = String::new();
        stdin()
            .read_line(&mut human_input)
            .expect("Failed to read line");

        // trim whitespace and convert to lower case
        let human_input: String = human_input.trim().to_lowercase();

        // Match human input
        match human_input.as_str() {
            "1" | "ok" | "yes" => return true,
            "2" | "no" | "n" => return false,
            _ => {
                println!("Invalid input, please select '1' or '2'.");
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_prints_agent_msg() {
        PrintCommand::AICall
            .print_agent_message("Managing Agent", "Testing, testing, processeng something");
    }
}
