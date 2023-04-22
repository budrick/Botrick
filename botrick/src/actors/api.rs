pub trait Actor: Sync + Send {
    fn process(&self, message: irc::proto::Message);
}
impl core::fmt::Debug for dyn Actor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Actor")
    }
}
