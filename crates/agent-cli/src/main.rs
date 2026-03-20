use agent_core::domain::session::Session;

fn main() {
    let session = Session::new();

    println!("agent-cli");
    println!("session initialized");
    println!("messages: {}", session.messages.len());
}
