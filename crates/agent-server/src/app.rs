use agent_core::ports::llm::LlmPort;
use agent_core::ports::storage::SessionStore;
use agent_core::usecases::load_state::load_state_from_store;
use agent_infra::llama::NullLlmAdapter;
use agent_infra::session_store_factory::SessionStoreFactory;

pub fn run() {
    let llm = NullLlmAdapter::new();

    let store = match SessionStoreFactory::from_env() {
        Ok(store) => store,
        Err(err) => {
            eprintln!("failed to build session store: {err}");
            return;
        }
    };

    let _llm_port: &dyn LlmPort = &llm;
    let _session_store: &dyn SessionStore = store.as_ref();

    let state = match load_state_from_store(store.as_ref()) {
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
