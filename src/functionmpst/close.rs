use crate::binary::End;
use crate::role::end::RoleEnd;
use crate::role::Role;
use crate::sessionmpst::SessionMpst;
use std::error::Error;

/// Closes a `SessionMpst`. Synchronises with all partners, and fails if one of the partners
/// has crashed.
pub fn close_mpst<R>(s: SessionMpst<End, End, RoleEnd, R>) -> Result<(), Box<dyn Error>>
where
    R: Role,
{
    s.session1.sender.send(()).unwrap_or(());
    s.session2.sender.send(()).unwrap_or(());

    s.session1.receiver.recv()?;
    s.session2.receiver.recv()?;

    Ok(())
}
