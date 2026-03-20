use crate::domain::session::Session;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppState {
    pub session: Session,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            session: Session::new(),
        }
    }
}
