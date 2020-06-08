///
/// ? Handle<...>
pub trait SessionManager: Sized // where
//     Self: SystemService,
{
    /// Information about an ongoing Session.
    ///
    type Session: Session;

    // ///
    // type Listener: Listener;

    fn reconfigure(&mut self);
}

///
/// ? can be polled until completion, producing a Session
pub trait Session: Sized // where
//     Self: Actor,
{
}
