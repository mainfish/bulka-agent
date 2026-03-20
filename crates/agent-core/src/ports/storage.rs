use crate::domain::session::Session;
use crate::errors::CoreResult;

pub trait SessionStore {
    fn load(&self) -> CoreResult<Session>;
    fn save(&self, session: &Session) -> CoreResult<()>;
}
