use crate::role::a::RoleA;
use crate::role::Role;
use crossbeam_channel::{bounded, Sender};

/// Gives the order to the `SessionMpst` related to A to execute its `session` field with B.
///
/// This `struct` should only be used in the `queue` field of the `SessionMpst` related to A.
#[derive(Debug)]
pub struct RoleADual<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleADual<R> {
    type Dual = RoleA<R::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = bounded::<R>(1);
        let (sender_dual, _) = bounded::<R::Dual>(1);

        (
            RoleADual {
                sender: sender_dual,
            },
            RoleA {
                sender: sender_normal,
            },
        )
    }

    #[doc(hidden)]
    fn head_str() -> String {
        String::from("RoleADual")
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }
}

/// Send a value of type `Role`. Always succeeds. Returns the continuation of the
/// queue `R`.
pub fn next_a_dual<R>(r: RoleADual<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}
