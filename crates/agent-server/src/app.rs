use agent_core::domain::state::AppState;

pub fn run() {
    let state = AppState::new();

    println!("agent-server");
    println!("server shell initialized");
    println!("messages: {}", state.session.messages.len());
}
