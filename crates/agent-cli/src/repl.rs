use agent_core::domain::command::AgentCommand;
use agent_core::domain::session::Session;
use agent_core::usecases::run_turn::run_turn;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("agent-cli");
    println!("type /exit to quit");

    let mut session = Session::new();

    loop {
        let command = read_command()?;

        match command {
            AgentCommand::Exit => {
                println!("Bye ✋🏼");
                break;
            }
            other => {
                run_turn(&mut session, other)?;

                println!("messages: {}", session.messages.len());

                if let Some(last) = session.messages.last() {
                    println!("last: {}", last.content);
                }
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
