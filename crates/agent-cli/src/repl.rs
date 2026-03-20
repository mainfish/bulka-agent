pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("agent-cli");
    println!("type /exit to quit");

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let trimmed = input.trim();
        match trimmed {
            "/exit" | ":q" => {
                println!("bye");
                break;
            }
            "" => continue,
            _ => {
                println!("not implemented: {trimmed}");
            }
        }
    }

    Ok(())
}
