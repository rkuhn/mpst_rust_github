use crossbeam_channel::{bounded, Sender};
use role::a_to_c::RoleAtoC;
use role::Role;

pub struct RoleCtoA<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleCtoA<R> {
    type Dual = RoleAtoC<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, _) = bounded::<R>(1);
        let (sender_dual, _) = bounded::<R::Dual>(1);

        return (
            RoleCtoA {
                sender: sender_dual,
            },
            RoleAtoC { sender: sender },
        );
    }
}

pub fn next_c_to_a<R>(r: RoleCtoA<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}