#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::{close, fork_with_thread_id, recv, send, End, Recv, Send, Session};
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::{
    bundle_fork_multi, choose, choose_mpst_multi_to_all, close_mpst, create_normal_role,
    create_recv_mpst_session, create_send_mpst_session, create_sessionmpst, offer, offer_mpst,
};

use std::error::Error;
use std::thread::JoinHandle;
use std::time::Duration;

// Create new SessionMpst for four participants
create_sessionmpst!(SessionMpstFour, 4);

// Create new roles
// normal
create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
create_normal_role!(RoleD, next_d, RoleDDual, next_d_dual);

// Create new send functions
// A
create_send_mpst_session!(
    send_mpst_a_to_b,
    RoleB,
    next_b,
    RoleA,
    SessionMpstFour,
    4,
    1
);
create_send_mpst_session!(
    send_mpst_a_to_c,
    RoleC,
    next_c,
    RoleA,
    SessionMpstFour,
    4,
    2
);
create_send_mpst_session!(
    send_mpst_a_to_d,
    RoleD,
    next_d,
    RoleA,
    SessionMpstFour,
    4,
    3
);
// B
create_send_mpst_session!(
    send_mpst_b_to_a,
    RoleA,
    next_a,
    RoleB,
    SessionMpstFour,
    4,
    1
);
create_send_mpst_session!(
    send_mpst_b_to_c,
    RoleC,
    next_c,
    RoleB,
    SessionMpstFour,
    4,
    2
);
create_send_mpst_session!(
    send_mpst_b_to_d,
    RoleD,
    next_d,
    RoleB,
    SessionMpstFour,
    4,
    3
);
// C
create_send_mpst_session!(
    send_mpst_c_to_a,
    RoleA,
    next_a,
    RoleC,
    SessionMpstFour,
    4,
    1
);
create_send_mpst_session!(
    send_mpst_c_to_b,
    RoleB,
    next_b,
    RoleC,
    SessionMpstFour,
    4,
    2
);
create_send_mpst_session!(
    send_mpst_c_to_d,
    RoleD,
    next_d,
    RoleC,
    SessionMpstFour,
    4,
    3
);
// D
create_send_mpst_session!(
    send_mpst_d_to_a,
    RoleA,
    next_a,
    RoleD,
    SessionMpstFour,
    4,
    1
);
create_send_mpst_session!(
    send_mpst_d_to_b,
    RoleB,
    next_b,
    RoleD,
    SessionMpstFour,
    4,
    2
);
create_send_mpst_session!(
    send_mpst_d_to_c,
    RoleC,
    next_c,
    RoleD,
    SessionMpstFour,
    4,
    3
);

// Create new recv functions and related types
// A
create_recv_mpst_session!(
    recv_mpst_a_to_b,
    RoleB,
    next_b,
    RoleA,
    SessionMpstFour,
    4,
    1
);
create_recv_mpst_session!(
    recv_mpst_a_to_c,
    RoleC,
    next_c,
    RoleA,
    SessionMpstFour,
    4,
    2
);
create_recv_mpst_session!(
    recv_mpst_a_to_d,
    RoleD,
    next_d,
    RoleA,
    SessionMpstFour,
    4,
    3
);
// B
create_recv_mpst_session!(
    recv_mpst_b_to_a,
    RoleA,
    next_a,
    RoleB,
    SessionMpstFour,
    4,
    1
);
create_recv_mpst_session!(
    recv_mpst_b_to_c,
    RoleC,
    next_c,
    RoleB,
    SessionMpstFour,
    4,
    2
);
create_recv_mpst_session!(
    recv_mpst_b_to_d,
    RoleD,
    next_d,
    RoleB,
    SessionMpstFour,
    4,
    3
);
// C
create_recv_mpst_session!(
    recv_mpst_c_to_a,
    RoleA,
    next_a,
    RoleC,
    SessionMpstFour,
    4,
    1
);
create_recv_mpst_session!(
    recv_mpst_c_to_b,
    RoleB,
    next_b,
    RoleC,
    SessionMpstFour,
    4,
    2
);
create_recv_mpst_session!(
    recv_mpst_c_to_d,
    RoleD,
    next_d,
    RoleC,
    SessionMpstFour,
    4,
    3
);
// D
create_recv_mpst_session!(
    recv_mpst_d_to_a,
    RoleA,
    next_a,
    RoleD,
    SessionMpstFour,
    4,
    1
);
create_recv_mpst_session!(
    recv_mpst_d_to_b,
    RoleB,
    next_b,
    RoleD,
    SessionMpstFour,
    4,
    2
);
create_recv_mpst_session!(
    recv_mpst_d_to_c,
    RoleC,
    next_c,
    RoleD,
    SessionMpstFour,
    4,
    3
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstFour, 4);

// Create fork function
bundle_fork_multi!(fork_mpst, fork_simple, SessionMpstFour, 4);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;

// Types
// Binary
// A
enum BranchingDforA {
    More(
        SessionMpstFour<
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursAtoD>>,
            RoleD<RoleD<RoleB<RoleB<RoleC<RoleC<RoleD<RoleEnd>>>>>>>,
            NameA,
        >,
    ),
    Done(SessionMpstFour<End, End, End, RoleEnd, NameA>),
}
type RecursAtoD = Recv<BranchingDforA, End>;
// B
enum BranchingDforB {
    More(
        SessionMpstFour<
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursBtoD>>,
            RoleD<RoleD<RoleA<RoleA<RoleC<RoleC<RoleD<RoleEnd>>>>>>>,
            NameB,
        >,
    ),
    Done(SessionMpstFour<End, End, End, RoleEnd, NameB>),
}
type RecursBtoD = Recv<BranchingDforB, End>;
// C
enum BranchingDforC {
    More(
        SessionMpstFour<
            Send<(), Recv<(), End>>,
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), RecursCtoD>>,
            RoleD<RoleD<RoleA<RoleA<RoleB<RoleB<RoleD<RoleEnd>>>>>>>,
            NameC,
        >,
    ),
    Done(SessionMpstFour<End, End, End, RoleEnd, NameC>),
}
type RecursCtoD = Recv<BranchingDforC, End>;
// D
type ChooseDforAtoD = Send<BranchingDforA, End>;
type ChooseDforBtoD = Send<BranchingDforB, End>;
type ChooseDforCtoD = Send<BranchingDforC, End>;

// Creating the MP sessions
type EndpointA = SessionMpstFour<End, End, RecursAtoD, RoleD<RoleEnd>, NameA>;
type EndpointB = SessionMpstFour<End, End, RecursBtoD, RoleD<RoleEnd>, NameB>;
type EndpointC = SessionMpstFour<End, End, RecursCtoD, RoleD<RoleEnd>, NameC>;
type EndpointD = SessionMpstFour<
    ChooseDforAtoD,
    ChooseDforBtoD,
    ChooseDforCtoD,
    RoleA<RoleB<RoleC<RoleEnd>>>,
    NameD,
>;

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_to_d, {
        BranchingDforA::Done(s) => {
            close_mpst_multi(s)
        },
        BranchingDforA::More(s) => {
            let (_, s) = recv_mpst_a_to_d(s)?;
            let s = send_mpst_a_to_d((), s);
            let (_, s) = recv_mpst_a_to_b(s)?;
            let s = send_mpst_a_to_b((), s);
            let (_, s) = recv_mpst_a_to_c(s)?;
            let s = send_mpst_a_to_c((), s);
            simple_five_endpoint_a(s)
        },
    })
}

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_to_d, {
        BranchingDforB::Done(s) => {
            close_mpst_multi(s)
        },
        BranchingDforB::More(s) => {
            let (_, s) = recv_mpst_b_to_d(s)?;
            let s = send_mpst_b_to_d((), s);
            let s = send_mpst_b_to_a((), s);
            let (_, s) = recv_mpst_b_to_a(s)?;
            let (_, s) = recv_mpst_b_to_c(s)?;
            let s = send_mpst_b_to_c((), s);
            simple_five_endpoint_b(s)
        },
    })
}

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_to_d, {
        BranchingDforC::Done(s) => {
            close_mpst_multi(s)
        },
        BranchingDforC::More(s) => {
            let (_, s) = recv_mpst_c_to_d(s)?;
            let s = send_mpst_c_to_d((), s);
            let s = send_mpst_c_to_a((), s);
            let (_, s) = recv_mpst_c_to_a(s)?;
            let s = send_mpst_c_to_b((), s);
            let (_, s) = recv_mpst_c_to_b(s)?;
            simple_five_endpoint_c(s)
        },
    })
}

fn simple_five_endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    recurs_d(s, SIZE)
}

fn recurs_d(s: EndpointD, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_d_to_a,
                send_mpst_d_to_b,
                send_mpst_d_to_c, =>
                BranchingDforA::Done,
                BranchingDforB::Done,
                BranchingDforC::Done, =>
                RoleA,
                RoleB,
                RoleC, =>
                RoleD,
                SessionMpstFour,
                4
            );

            close_mpst_multi(s)
        }
        i => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_d_to_a,
                send_mpst_d_to_b,
                send_mpst_d_to_c, =>
                BranchingDforA::More,
                BranchingDforB::More,
                BranchingDforC::More, =>
                RoleA,
                RoleB,
                RoleC, =>
                RoleD,
                SessionMpstFour,
                4
            );

            let s = send_mpst_d_to_a((), s);
            let (_, s) = recv_mpst_d_to_a(s)?;
            let s = send_mpst_d_to_b((), s);
            let (_, s) = recv_mpst_d_to_b(s)?;
            let s = send_mpst_d_to_c((), s);
            let (_, s) = recv_mpst_d_to_c(s)?;

            recurs_d(s, i - 1)
        }
    }
}

fn all_mpst() -> Result<(), Box<dyn Error>> {
    let (thread_a, thread_b, thread_c, thread_d) = fork_mpst(
        black_box(simple_five_endpoint_a),
        black_box(simple_five_endpoint_b),
        black_box(simple_five_endpoint_c),
        black_box(simple_five_endpoint_d),
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();

    Ok(())
}

/////////////////////////
// A
enum BinaryA {
    More(Recv<(), Send<(), RecursA>>),
    Done(End),
}
type RecursA = Recv<BinaryA, End>;
fn binary_b_to_c(s: RecursA) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        BinaryA::More(s) => {
            let (_, s) = recv(s)?;
            let s = send((), s);
            binary_b_to_c(s)
        },
        BinaryA::Done(s) => {
            close(s)
        },
    })
}

// B
type RecursB = <RecursA as Session>::Dual;
fn binary_c_to_b(s: RecursB, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose!(BinaryA::Done, s);
            close(s)
        }
        i => {
            let s = choose!(BinaryA::More, s);
            let s = send((), s);
            let (_, s) = recv(s)?;
            binary_c_to_b(s, i - 1)
        }
    }
}

fn all_binaries() -> Result<(), Box<dyn Error>> {
    let mut threads = Vec::new();

    for _ in 0..6 { // 12
        let (thread_b_to_c, s_b_to_c): (JoinHandle<()>, RecursB) =
            fork_with_thread_id(black_box(binary_b_to_c));

        threads.push((s_b_to_c, thread_b_to_c));
    }

    for elt in threads {
        binary_c_to_b(black_box(elt.0), SIZE).unwrap();
        elt.1.join().unwrap();
    }

    Ok(())
}

/////////////////////////

static SIZE: i64 = 100;

fn long_simple_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("long four simple protocol MPST {}", SIZE), |b| {
        b.iter(|| all_mpst())
    });
}

fn long_simple_protocol_binary(c: &mut Criterion) {
    c.bench_function(&format!("long four simple protocol binary {}", SIZE), |b| {
        b.iter(|| all_binaries())
    });
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(150, 0))
}

criterion_group! {
    name = long_four_simple_protocols;
    config = long_warmup();
    targets = long_simple_protocol_mpst, long_simple_protocol_binary
}
criterion_main!(long_four_simple_protocols);