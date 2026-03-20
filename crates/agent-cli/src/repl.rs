use agent_core::domain::command::AgentCommand;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("agent-cli");
    println!("type /exit to quit");

    loop {
        let command = read_command()?;

        match command {
            AgentCommand::Exit => {
                println!("bye ✋🏼");
                break;
            }
            AgentCommand::ClearSession => {
                println!("not implemented: clear session");
            }
            AgentCommand::UserPrompt(prompt) => {
                println!("not implemented: {prompt}");
            }
        }
    }

    Ok(())
}

fn read_command() -> Result<AgentCommand, Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let trimmed = input.trim();

    let command = match trimmed {
        "/exit" | ":q" => AgentCommand::Exit,
        "/clear" => AgentCommand::ClearSession,
        "" => return read_command(),
        _ => AgentCommand::UserPrompt(trimmed.to_string()),
    };

    Ok(command)
}
