use crate::domain::state::AppState;
use crate::errors::CoreResult;
use crate::ports::storage::SessionStore;

pub fn load_state_from_store(store: &dyn SessionStore) -> CoreResult<AppState> {
    let session = store.load()?;
    Ok(AppState { session })
}
