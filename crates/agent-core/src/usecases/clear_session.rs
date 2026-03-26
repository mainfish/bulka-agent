use crate::domain::state::AppState;
use crate::errors::CoreResult;

pub fn clear_session(state: &mut AppState) -> CoreResult<()> {
    state.session.messages.clear();
    Ok(())
}
