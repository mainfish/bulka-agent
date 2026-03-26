use agent_core::ports::llm::LlmPort;
use agent_core::ports::storage::SessionStore;
use agent_core::usecases::load_state::load_state_from_store;
use agent_infra::llama::NullLlmAdapter;
use agent_infra::storage::NullSessionStore;

pub fn run() {
    let llm = NullLlmAdapter::new();
    let store = NullSessionStore::new();

    let _llm_port: &dyn LlmPort = &llm;
    let _session_store: &dyn SessionStore = &store;

    let state = match load_state_from_store(&store) {
        Ok(state) => state,
        Err(err) => {
            eprintln!("failed to load state: {err}");
            return;
        }
    };

    println!("agent-server");
    println!("server shell initialized");
    println!("llm adapter wired");
    println!("tool adapter wired");
    println!("session store wired");
    println!("messages: {}", state.session.messages.len());
}
