use crate::role::all_to_a::RoleAlltoA;
use crate::role::Role;
use crossbeam_channel::{bounded, Sender};

/// Gives the order to the `SessionMpst` related to A to execute its `session`
/// fields with every other processes.
///
/// This `struct` is used for branching without `enum`. See test `usecase`.
#[derive(Debug)]
pub struct RoleAtoAll<R1: Role, R2: Role> {
    pub sender1: Sender<R1::Dual>,
    pub sender2: Sender<R2::Dual>,
}

impl<R1: Role, R2: Role> Role for RoleAtoAll<R1, R2> {
    type Dual = RoleAlltoA<R1::Dual, R2::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal_1, _) = bounded::<R1>(1);
        let (sender_normal_2, _) = bounded::<R2>(1);
        let (sender_dual_1, _) = bounded::<R1::Dual>(1);
        let (sender_dual_2, _) = bounded::<R2::Dual>(1);

        (
            RoleAtoAll {
                sender1: sender_dual_1,
                sender2: sender_dual_2,
            },
            RoleAlltoA {
                sender1: sender_normal_1,
                sender2: sender_normal_2,
            },
        )
    }

    #[doc(hidden)]
    fn head_str() -> String {
        String::from("RoleAtoAll")
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        format!(
            "{}<{}> + {}<{}>",
            R1::head_str(),
            R1::tail_str(),
            R2::head_str(),
            R2::tail_str()
        )
    }
}

/// Send two values of type `Role` from A, which may be different. Always succeeds. Returns the continuation of the
/// queue `(R1, R2)`.
pub fn next_a_to_all<R1, R2>(r: RoleAtoAll<R1, R2>) -> (R1, R2)
where
    R1: Role,
    R2: Role,
{
    let (here1, there1) = R1::new();
    let (here2, there2) = R2::new();
    r.sender1.send(there1).unwrap_or(());
    r.sender2.send(there2).unwrap_or(());
    (here1, here2)
}
