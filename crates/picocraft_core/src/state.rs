#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub enum State {
    #[default]
    Handshake,
    Status,
    Login,
    Configuration,
    Play,
}
