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

        //Decide on the print color
        let statement_color: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };

        // Print the agent statement
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}: ", agent_pos);

        // Make selected color
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{} ", agent_statement);

        // Reset the color
        stdout.execute(ResetColor).unwrap();
    }
}

// get user request
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    // print the question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    // reset the color
    stdout.execute(ResetColor).unwrap();

    //Read user input
    let mut user_response: String = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Did not enter a correct string");

    //Trim whitespaces and return

    return user_response.trim().to_string();
}

//get user response that code is safe to execute

pub fn confirm_safe_code() -> bool {
    let mut stdout: std::io::Stdout = stdout();

    loop {
        // print the question in a specific color
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!("");
        print!("WARNING: YOU are about to run code written by an AI agent. ");
        println!("Review your code and confirm that you wish to continue.");

        // reset the color
        stdout.execute(ResetColor).unwrap();

        // Present Options with defferent colors
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] All Good");
        stdout.execute(SetForegroundColor(Color::DarkRed)).unwrap();
        println!("[2] Lets stop this project");

        // reset the color
        stdout.execute(ResetColor).unwrap();

        //Read user input
        let mut human_string: String = String::new();
        stdin()
            .read_line(&mut human_string)
            .expect("Failed to read respopnse");

        //Trim whitespaces and convert to lowercase

        let human_response: String = human_string.trim().to_lowercase();

        //Match response
        match human_response.as_str() {
            "1" | "ok" | "y" => return true,
            "2" | "no" | "n" => return false,
            _ => {
                println!("Invalid input. Please select '1', or '2'")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prints_agent_msg() {
        PrintCommand::AICall.print_agent_message("Managing agent", "testing, testing, 1, 2, 3");
    }
}
