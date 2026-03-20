use agent_core::domain::session::Session;

pub fn run() {
    let session = Session::new();

    println!("agent-server");
    println!("server shell initialized");
    println!("messages: {}", session.messages.len());
}
