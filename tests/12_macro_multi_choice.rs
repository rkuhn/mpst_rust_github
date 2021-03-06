// Test for parametrisation on the number of roles
extern crate mpstthree;
use rand::{thread_rng, Rng};

use mpstthree::binary::{End, Recv, Send, Session};
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::{
    close_mpst, create_broadcast_role, create_choose_mpst_session_multi_left,
    create_choose_mpst_session_multi_right, create_choose_type_multi, create_normal_role,
    create_offer_mpst_session_multi, create_offer_type_multi, create_recv_mpst_all_session,
    create_recv_mpst_session, create_send_mpst_session, create_sessionmpst, fork_mpst_multi,
    fork_simple_multi,
};
use std::error::Error;

// Create new SessionMpst for three participants
create_sessionmpst!(SessionMpst, 3);

// Create new roles
// normal
create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
create_normal_role!(RoleD, next_d, RoleDDual, next_d_dual);
// broadcast
create_broadcast_role!(RoleAlltoD, next_all_to_d, RoleDtoAll, next_d_to_all);

// Create new send functions
create_send_mpst_session!(send_mpst_d_to_a, RoleA, next_a, RoleD, SessionMpst, 3, 1);
create_send_mpst_session!(send_mpst_a_to_d, RoleD, next_d, RoleA, SessionMpst, 3, 2);
create_send_mpst_session!(send_mpst_d_to_b, RoleB, next_b, RoleD, SessionMpst, 3, 2);
create_send_mpst_session!(send_mpst_b_to_a, RoleA, next_a, RoleB, SessionMpst, 3, 1);
create_send_mpst_session!(send_mpst_a_to_b, RoleB, next_b, RoleA, SessionMpst, 3, 1);

// Create new recv functions and related types
// normal
create_recv_mpst_session!(recv_mpst_d_to_a, RoleA, next_a, RoleD, SessionMpst, 3, 1);
create_recv_mpst_session!(recv_mpst_a_to_d, RoleD, next_d, RoleA, SessionMpst, 3, 2);
create_recv_mpst_session!(recv_mpst_b_to_d, RoleD, next_d, RoleB, SessionMpst, 3, 2);
create_recv_mpst_session!(recv_mpst_b_to_a, RoleA, next_a, RoleB, SessionMpst, 3, 1);
create_recv_mpst_session!(recv_mpst_a_to_b, RoleB, next_b, RoleA, SessionMpst, 3, 1);
// broadcast
create_recv_mpst_all_session!(
    recv_mpst_b_all_to_d,
    RoleAlltoD,
    next_all_to_d,
    RoleB,
    SessionMpst,
    3,
    2
);
create_recv_mpst_all_session!(
    recv_mpst_a_all_to_d,
    RoleAlltoD,
    next_all_to_d,
    RoleA,
    SessionMpst,
    3,
    2
);

close_mpst!(close_mpst_multi, SessionMpst, 3);

create_offer_type_multi!(OfferMpstMultiThree, SessionMpst, 3, 2);
create_choose_type_multi!(ChooseMpstThree, SessionMpst, 3, 2);

create_offer_mpst_session_multi!(
    offer_mpst_session_a_to_d,
    OfferMpstMultiThree,
    RoleAlltoD,
    recv_mpst_a_all_to_d,
    RoleA,
    SessionMpst,
    3,
    2
);

create_offer_mpst_session_multi!(
    offer_mpst_session_b_to_d,
    OfferMpstMultiThree,
    RoleAlltoD,
    recv_mpst_b_all_to_d,
    RoleB,
    SessionMpst,
    3,
    2
);

create_choose_mpst_session_multi_left!(
    choose_left_mpst_session_d_to_all,
    ChooseMpstThree,
    RoleDtoAll,
    next_d_to_all,
    RoleD,
    SessionMpst,
    3
);

create_choose_mpst_session_multi_right!(
    choose_right_mpst_session_d_to_all,
    ChooseMpstThree,
    RoleDtoAll,
    next_d_to_all,
    RoleD,
    SessionMpst,
    3
);

fork_simple_multi!(fork_simple, SessionMpst, 3);
fork_mpst_multi!(fork_mpst, fork_simple, SessionMpst, 3);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameD = RoleD<RoleEnd>;

// Types
type AtoCClose = End;
type AtoBClose = End;
type AtoCVideo<N> = Recv<N, Send<N, End>>;
type AtoBVideo<N> = Send<N, Recv<N, End>>;

type BtoAClose = <AtoBClose as Session>::Dual;
type BtoCClose = End;
type BtoAVideo<N> = <AtoBVideo<N> as Session>::Dual;

type CtoBClose = <BtoCClose as Session>::Dual;
type CtoAClose = <AtoCClose as Session>::Dual;
type CtoAVideo<N> = <AtoCVideo<N> as Session>::Dual;

/// Queues
type QueueAEnd = RoleEnd;
type QueueAEndDual = <QueueAEnd as Role>::Dual;
type QueueAVideo = RoleD<RoleB<RoleB<RoleD<RoleEnd>>>>;
type QueueAVideoDual = <QueueAVideo as Role>::Dual;
type QueueAFull = RoleD<RoleD<RoleAlltoD<RoleEnd, RoleEnd>>>;

type QueueBEnd = RoleEnd;
type QueueBEndDual = <QueueBEnd as Role>::Dual;
type QueueBVideo = RoleA<RoleA<RoleEnd>>;
type QueueBVideoDual = <QueueBVideo as Role>::Dual;
type QueueBFull = RoleAlltoD<RoleEnd, RoleEnd>;

type QueueCEnd = RoleEnd;
type QueueCVideo = RoleA<RoleA<RoleEnd>>;
type QueueCChoice = RoleDtoAll<QueueCVideo, QueueCEnd>;
type QueueCFull = RoleA<RoleA<QueueCChoice>>;

/// Creating the MP sessions
/// For C
type ChooseCtoA<N> = ChooseMpstThree<
    BtoAVideo<N>,
    CtoAVideo<N>,
    BtoAClose,
    CtoAClose,
    QueueAVideoDual,
    QueueAEnd,
    RoleADual<RoleEnd>,
>;
type ChooseCtoB<N> = ChooseMpstThree<
    AtoBVideo<N>,
    CtoBClose,
    AtoBClose,
    CtoBClose,
    QueueBVideoDual,
    QueueBEnd,
    RoleBDual<RoleEnd>,
>;
type InitC<N> = Send<N, Recv<N, ChooseCtoA<N>>>;
type EndpointCFull<N> = SessionMpst<InitC<N>, ChooseCtoB<N>, QueueCFull, NameD>;

/// For A
type EndpointAVideo<N> = SessionMpst<AtoBVideo<N>, AtoCVideo<N>, QueueAVideo, NameA>;
type EndpointAEnd = SessionMpst<AtoBClose, AtoCClose, QueueAEnd, NameA>;

type OfferA<N> = OfferMpstMultiThree<
    AtoBVideo<N>,
    AtoCVideo<N>,
    AtoBClose,
    AtoCClose,
    QueueAVideo,
    QueueAEnd,
    NameA,
>;
type InitA<N> = Recv<N, Send<N, OfferA<N>>>;
type EndpointAFull<N> = SessionMpst<End, InitA<N>, QueueAFull, NameA>;

/// For B
type EndpointBVideo<N> = SessionMpst<BtoAVideo<N>, BtoCClose, QueueBVideo, NameB>;
type EndpointBEnd = SessionMpst<BtoAClose, BtoCClose, QueueBEnd, NameB>;

type OfferB<N> = OfferMpstMultiThree<
    BtoAVideo<N>,
    BtoCClose,
    BtoAClose,
    BtoCClose,
    QueueBVideo,
    QueueBEnd,
    NameB,
>;
type EndpointBFull<N> = SessionMpst<End, OfferB<N>, QueueBFull, NameB>;

/// Functions related to endpoints
fn server(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_b_to_d(
        s,
        |s: EndpointBVideo<i32>| {
            let (request, s) = recv_mpst_b_to_a(s)?;
            let s = send_mpst_b_to_a(request + 1, s);

            close_mpst_multi(s)?;

            Ok(())
        },
        |s: EndpointBEnd| {
            close_mpst_multi(s)?;

            Ok(())
        },
    )
}

fn authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_a_to_d(s)?;
    let s = send_mpst_a_to_d(id + 1, s);

    offer_mpst_session_a_to_d(
        s,
        |s: EndpointAVideo<i32>| {
            let (request, s) = recv_mpst_a_to_d(s)?;
            let s = send_mpst_a_to_b(request + 1, s);
            let (video, s) = recv_mpst_a_to_b(s)?;
            let s = send_mpst_a_to_d(video + 1, s);

            assert_eq!(request, id + 1);
            assert_eq!(video, id + 3);

            close_mpst_multi(s)?;
            Ok(())
        },
        |s: EndpointAEnd| {
            close_mpst_multi(s)?;

            Ok(())
        },
    )
}

fn client_video(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    {
        let mut rng = thread_rng();
        let id: i32 = rng.gen();

        let s = send_mpst_d_to_a(id, s);
        let (accept, s) = recv_mpst_d_to_a(s)?;

        assert_eq!(accept, id + 1);

        let s = choose_left_mpst_session_d_to_all::<
            AtoBVideo<i32>,
            AtoCVideo<i32>,
            BtoAClose,
            CtoAClose,
            BtoCClose,
            AtoCClose,
            QueueAVideoDual,
            QueueAEndDual,
            <NameA as Role>::Dual,
            QueueBVideoDual,
            QueueBEndDual,
            <NameB as Role>::Dual,
            QueueCVideo,
            QueueCEnd,
        >(s);

        let s = send_mpst_d_to_a(accept, s);
        let (result, s) = recv_mpst_d_to_a(s)?;

        assert_eq!(result, accept + 3);

        close_mpst_multi(s)?;
    }
    Ok(())
}

fn client_close(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    {
        let mut rng = thread_rng();
        let id: i32 = rng.gen();

        let s = send_mpst_d_to_a(id, s);
        let (accept, s) = recv_mpst_d_to_a(s)?;

        assert_eq!(accept, id + 1);

        let s = choose_right_mpst_session_d_to_all::<
            AtoBVideo<i32>,
            AtoCVideo<i32>,
            BtoAClose,
            CtoAClose,
            BtoCClose,
            AtoCClose,
            QueueAVideoDual,
            QueueAEndDual,
            <NameA as Role>::Dual,
            QueueBVideoDual,
            QueueBEndDual,
            <NameB as Role>::Dual,
            QueueCVideo,
            QueueCEnd,
        >(s);

        close_mpst_multi(s)?;
    }
    Ok(())
}

////////////////////////////////////////

#[test]
fn test_new_choice_full() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_pawn, thread_d) = fork_mpst(authenticator, server, client_video);

            assert!(thread_a.is_ok());
            assert!(thread_pawn.is_ok());
            assert!(thread_d.is_ok());
        }
        Ok(())
    }()
    .is_ok());
}

#[test]
fn test_new_choice_close() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test end branch.
        {
            let (thread_a, thread_pawn, thread_d) = fork_mpst(authenticator, server, client_close);

            assert!(thread_a.is_ok());
            assert!(thread_pawn.is_ok());
            assert!(thread_d.is_ok());
        }

        Ok(())
    }()
    .is_ok());
}
