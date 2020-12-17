// // Test for parametrisation on the number of roles
// use rand::{thread_rng, Rng};

// use mpstthree::binary::{End, Recv, Send, Session};
// use mpstthree::role::end::RoleEnd;
// use mpstthree::role::Role;
// use mpstthree::{
//     bundle_fork_multi, close_mpst, create_broadcast_role, create_choose_mpst_session_multi_both,
//     create_choose_type_multi, create_normal_role, create_offer_mpst_session_multi,
//     create_offer_type_multi, create_recv_mpst_all_session, create_recv_mpst_session,
//     create_send_mpst_session, create_sessionmpst,
// };
// use std::error::Error;

// // Create new SessionMpst for three participants
// create_sessionmpst!(SessionMpstFive, 5);

// // Create new roles
// // normal
// create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
// create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
// create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
// create_normal_role!(RoleD, next_d, RoleDDual, next_d_dual);
// create_normal_role!(RoleE, next_e, RoleEDual, next_e_dual);
// // broadcast
// create_broadcast_role!(RoleAlltoA, next_all_to_a, RoleAtoAll, next_a_to_all);
// create_broadcast_role!(RoleAlltoB, next_all_to_b, RoleBtoAll, next_b_to_all);
// create_broadcast_role!(RoleAlltoC, next_all_to_c, RoleCtoAll, next_c_to_all);
// create_broadcast_role!(RoleAlltoD, next_all_to_d, RoleDtoAll, next_d_to_all);
// create_broadcast_role!(RoleAlltoE, next_all_to_e, RoleEtoAll, next_e_to_all);

// // Create new send functions
// // A
// create_send_mpst_session!(
//     send_mpst_b_to_a,
//     RoleA,
//     next_a,
//     RoleB,
//     SessionMpstFive,
//     5,
//     1
// );
// create_send_mpst_session!(
//     send_mpst_c_to_a,
//     RoleA,
//     next_a,
//     RoleC,
//     SessionMpstFive,
//     5,
//     1
// );
// create_send_mpst_session!(
//     send_mpst_d_to_a,
//     RoleA,
//     next_a,
//     RoleD,
//     SessionMpstFive,
//     5,
//     1
// );
// create_send_mpst_session!(
//     send_mpst_e_to_a,
//     RoleA,
//     next_a,
//     RoleE,
//     SessionMpstFive,
//     5,
//     1
// );
// // B
// create_send_mpst_session!(
//     send_mpst_a_to_b,
//     RoleB,
//     next_b,
//     RoleA,
//     SessionMpstFive,
//     5,
//     1
// );
// create_send_mpst_session!(
//     send_mpst_c_to_b,
//     RoleB,
//     next_b,
//     RoleC,
//     SessionMpstFive,
//     5,
//     2
// );
// create_send_mpst_session!(
//     send_mpst_d_to_b,
//     RoleB,
//     next_b,
//     RoleD,
//     SessionMpstFive,
//     5,
//     2
// );
// create_send_mpst_session!(
//     send_mpst_e_to_b,
//     RoleB,
//     next_b,
//     RoleE,
//     SessionMpstFive,
//     5,
//     2
// );
// // C
// create_send_mpst_session!(
//     send_mpst_a_to_c,
//     RoleC,
//     next_c,
//     RoleA,
//     SessionMpstFive,
//     5,
//     2
// );
// create_send_mpst_session!(
//     send_mpst_b_to_c,
//     RoleC,
//     next_c,
//     RoleB,
//     SessionMpstFive,
//     5,
//     2
// );
// create_send_mpst_session!(
//     send_mpst_d_to_c,
//     RoleC,
//     next_c,
//     RoleD,
//     SessionMpstFive,
//     5,
//     3
// );
// create_send_mpst_session!(
//     send_mpst_e_to_c,
//     RoleC,
//     next_c,
//     RoleE,
//     SessionMpstFive,
//     5,
//     3
// );
// // D
// create_send_mpst_session!(
//     send_mpst_a_to_d,
//     RoleD,
//     next_d,
//     RoleA,
//     SessionMpstFive,
//     5,
//     3
// );
// create_send_mpst_session!(
//     send_mpst_b_to_d,
//     RoleD,
//     next_d,
//     RoleB,
//     SessionMpstFive,
//     5,
//     3
// );
// create_send_mpst_session!(
//     send_mpst_c_to_d,
//     RoleD,
//     next_d,
//     RoleC,
//     SessionMpstFive,
//     5,
//     3
// );
// create_send_mpst_session!(
//     send_mpst_e_to_d,
//     RoleD,
//     next_d,
//     RoleE,
//     SessionMpstFive,
//     5,
//     4
// );
// // E
// create_send_mpst_session!(
//     send_mpst_a_to_e,
//     RoleE,
//     next_e,
//     RoleA,
//     SessionMpstFive,
//     5,
//     4
// );
// create_send_mpst_session!(
//     send_mpst_b_to_e,
//     RoleE,
//     next_e,
//     RoleB,
//     SessionMpstFive,
//     5,
//     4
// );
// create_send_mpst_session!(
//     send_mpst_c_to_e,
//     RoleE,
//     next_e,
//     RoleC,
//     SessionMpstFive,
//     5,
//     4
// );
// create_send_mpst_session!(
//     send_mpst_d_to_e,
//     RoleE,
//     next_e,
//     RoleD,
//     SessionMpstFive,
//     5,
//     4
// );

// // Create new recv functions and related types
// // normal
// // A
// create_recv_mpst_session!(
//     recv_mpst_b_to_a,
//     RoleA,
//     next_a,
//     RoleB,
//     SessionMpstFive,
//     5,
//     1
// );
// create_recv_mpst_session!(
//     recv_mpst_c_to_a,
//     RoleA,
//     next_a,
//     RoleC,
//     SessionMpstFive,
//     5,
//     1
// );
// create_recv_mpst_session!(
//     recv_mpst_d_to_a,
//     RoleA,
//     next_a,
//     RoleD,
//     SessionMpstFive,
//     5,
//     1
// );
// create_recv_mpst_session!(
//     recv_mpst_e_to_a,
//     RoleA,
//     next_a,
//     RoleE,
//     SessionMpstFive,
//     5,
//     1
// );
// // B
// create_recv_mpst_session!(
//     recv_mpst_a_to_b,
//     RoleB,
//     next_b,
//     RoleA,
//     SessionMpstFive,
//     5,
//     1
// );
// create_recv_mpst_session!(
//     recv_mpst_c_to_b,
//     RoleB,
//     next_b,
//     RoleC,
//     SessionMpstFive,
//     5,
//     2
// );
// create_recv_mpst_session!(
//     recv_mpst_d_to_b,
//     RoleB,
//     next_b,
//     RoleD,
//     SessionMpstFive,
//     5,
//     2
// );
// create_recv_mpst_session!(
//     recv_mpst_e_to_b,
//     RoleB,
//     next_b,
//     RoleE,
//     SessionMpstFive,
//     5,
//     2
// );
// // C
// create_recv_mpst_session!(
//     recv_mpst_a_to_c,
//     RoleC,
//     next_c,
//     RoleA,
//     SessionMpstFive,
//     5,
//     2
// );
// create_recv_mpst_session!(
//     recv_mpst_b_to_c,
//     RoleC,
//     next_c,
//     RoleB,
//     SessionMpstFive,
//     5,
//     2
// );
// create_recv_mpst_session!(
//     recv_mpst_d_to_c,
//     RoleC,
//     next_c,
//     RoleD,
//     SessionMpstFive,
//     5,
//     3
// );
// create_recv_mpst_session!(
//     recv_mpst_e_to_c,
//     RoleC,
//     next_c,
//     RoleE,
//     SessionMpstFive,
//     5,
//     3
// );
// // D
// create_recv_mpst_session!(
//     recv_mpst_a_to_d,
//     RoleD,
//     next_d,
//     RoleA,
//     SessionMpstFive,
//     5,
//     3
// );
// create_recv_mpst_session!(
//     recv_mpst_b_to_d,
//     RoleD,
//     next_d,
//     RoleB,
//     SessionMpstFive,
//     5,
//     3
// );
// create_recv_mpst_session!(
//     recv_mpst_c_to_d,
//     RoleD,
//     next_d,
//     RoleC,
//     SessionMpstFive,
//     5,
//     3
// );
// create_recv_mpst_session!(
//     recv_mpst_e_to_d,
//     RoleD,
//     next_d,
//     RoleE,
//     SessionMpstFive,
//     5,
//     4
// );
// // E
// create_recv_mpst_session!(
//     recv_mpst_a_to_e,
//     RoleE,
//     next_e,
//     RoleA,
//     SessionMpstFive,
//     5,
//     4
// );
// create_recv_mpst_session!(
//     recv_mpst_b_to_e,
//     RoleE,
//     next_e,
//     RoleB,
//     SessionMpstFive,
//     5,
//     4
// );
// create_recv_mpst_session!(
//     recv_mpst_c_to_e,
//     RoleE,
//     next_e,
//     RoleC,
//     SessionMpstFive,
//     5,
//     4
// );
// create_recv_mpst_session!(
//     recv_mpst_d_to_e,
//     RoleE,
//     next_e,
//     RoleD,
//     SessionMpstFive,
//     5,
//     4
// );

// // broadcast
// // A
// create_recv_mpst_all_session!(
//     recv_mpst_a_all_to_b,
//     RoleAlltoB,
//     next_all_to_b,
//     RoleA,
//     SessionMpstFive,
//     5,
//     1
// );
// create_recv_mpst_all_session!(
//     recv_mpst_a_all_to_c,
//     RoleAlltoC,
//     next_all_to_c,
//     RoleA,
//     SessionMpstFive,
//     5,
//     2
// );
// create_recv_mpst_all_session!(
//     recv_mpst_a_all_to_d,
//     RoleAlltoD,
//     next_all_to_d,
//     RoleA,
//     SessionMpstFive,
//     5,
//     3
// );
// create_recv_mpst_all_session!(
//     recv_mpst_a_all_to_e,
//     RoleAlltoE,
//     next_all_to_e,
//     RoleA,
//     SessionMpstFive,
//     5,
//     4
// );
// // B
// create_recv_mpst_all_session!(
//     recv_mpst_b_all_to_a,
//     RoleAlltoA,
//     next_all_to_a,
//     RoleB,
//     SessionMpstFive,
//     5,
//     1
// );
// create_recv_mpst_all_session!(
//     recv_mpst_b_all_to_c,
//     RoleAlltoC,
//     next_all_to_c,
//     RoleB,
//     SessionMpstFive,
//     5,
//     2
// );
// create_recv_mpst_all_session!(
//     recv_mpst_b_all_to_d,
//     RoleAlltoD,
//     next_all_to_d,
//     RoleB,
//     SessionMpstFive,
//     5,
//     3
// );
// create_recv_mpst_all_session!(
//     recv_mpst_b_all_to_e,
//     RoleAlltoE,
//     next_all_to_e,
//     RoleB,
//     SessionMpstFive,
//     5,
//     4
// );
// // C
// create_recv_mpst_all_session!(
//     recv_mpst_c_all_to_a,
//     RoleAlltoA,
//     next_all_to_a,
//     RoleC,
//     SessionMpstFive,
//     5,
//     1
// );
// create_recv_mpst_all_session!(
//     recv_mpst_c_all_to_b,
//     RoleAlltoB,
//     next_all_to_b,
//     RoleC,
//     SessionMpstFive,
//     5,
//     2
// );
// create_recv_mpst_all_session!(
//     recv_mpst_c_all_to_d,
//     RoleAlltoD,
//     next_all_to_d,
//     RoleC,
//     SessionMpstFive,
//     5,
//     3
// );
// create_recv_mpst_all_session!(
//     recv_mpst_c_all_to_e,
//     RoleAlltoE,
//     next_all_to_e,
//     RoleC,
//     SessionMpstFive,
//     5,
//     4
// );
// // D
// create_recv_mpst_all_session!(
//     recv_mpst_d_all_to_a,
//     RoleAlltoA,
//     next_all_to_a,
//     RoleD,
//     SessionMpstFive,
//     5,
//     1
// );
// create_recv_mpst_all_session!(
//     recv_mpst_d_all_to_b,
//     RoleAlltoB,
//     next_all_to_b,
//     RoleD,
//     SessionMpstFive,
//     5,
//     2
// );
// create_recv_mpst_all_session!(
//     recv_mpst_d_all_to_c,
//     RoleAlltoC,
//     next_all_to_c,
//     RoleD,
//     SessionMpstFive,
//     5,
//     3
// );
// create_recv_mpst_all_session!(
//     recv_mpst_d_all_to_e,
//     RoleAlltoE,
//     next_all_to_e,
//     RoleD,
//     SessionMpstFive,
//     5,
//     4
// );
// // E
// create_recv_mpst_all_session!(
//     recv_mpst_e_all_to_a,
//     RoleAlltoA,
//     next_all_to_a,
//     RoleE,
//     SessionMpstFive,
//     5,
//     1
// );
// create_recv_mpst_all_session!(
//     recv_mpst_e_all_to_b,
//     RoleAlltoB,
//     next_all_to_b,
//     RoleE,
//     SessionMpstFive,
//     5,
//     2
// );
// create_recv_mpst_all_session!(
//     recv_mpst_e_all_to_c,
//     RoleAlltoC,
//     next_all_to_c,
//     RoleE,
//     SessionMpstFive,
//     5,
//     3
// );
// create_recv_mpst_all_session!(
//     recv_mpst_e_all_to_d,
//     RoleAlltoD,
//     next_all_to_d,
//     RoleE,
//     SessionMpstFive,
//     5,
//     4
// );

// // Create Offer and Choose types
// create_offer_type_multi!(OfferMpstMultiFive, SessionMpstFive, 5);
// create_choose_type_multi!(ChooseMpstFive, SessionMpstFive, 5);

// // Create offer functions
// // A
// create_offer_mpst_session_multi!(
//     offer_mpst_session_a_to_b,
//     OfferMpstMultiFive,
//     RoleAlltoB,
//     recv_mpst_a_all_to_b,
//     RoleA,
//     SessionMpstFive,
//     5,
//     1
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_a_to_c,
//     OfferMpstMultiFive,
//     RoleAlltoC,
//     recv_mpst_a_all_to_c,
//     RoleA,
//     SessionMpstFive,
//     5,
//     2
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_a_to_d,
//     OfferMpstMultiFive,
//     RoleAlltoD,
//     recv_mpst_a_all_to_d,
//     RoleA,
//     SessionMpstFive,
//     5,
//     3
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_a_to_e,
//     OfferMpstMultiFive,
//     RoleAlltoE,
//     recv_mpst_a_all_to_e,
//     RoleA,
//     SessionMpstFive,
//     5,
//     4
// );
// // B
// create_offer_mpst_session_multi!(
//     offer_mpst_session_b_to_a,
//     OfferMpstMultiFive,
//     RoleAlltoA,
//     recv_mpst_b_all_to_a,
//     RoleB,
//     SessionMpstFive,
//     5,
//     1
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_b_to_c,
//     OfferMpstMultiFive,
//     RoleAlltoC,
//     recv_mpst_b_all_to_c,
//     RoleB,
//     SessionMpstFive,
//     5,
//     2
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_b_to_d,
//     OfferMpstMultiFive,
//     RoleAlltoD,
//     recv_mpst_b_all_to_d,
//     RoleB,
//     SessionMpstFive,
//     5,
//     3
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_b_to_e,
//     OfferMpstMultiFive,
//     RoleAlltoE,
//     recv_mpst_b_all_to_e,
//     RoleB,
//     SessionMpstFive,
//     5,
//     4
// );
// // C
// create_offer_mpst_session_multi!(
//     offer_mpst_session_c_to_a,
//     OfferMpstMultiFive,
//     RoleAlltoA,
//     recv_mpst_c_all_to_a,
//     RoleC,
//     SessionMpstFive,
//     5,
//     1
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_c_to_b,
//     OfferMpstMultiFive,
//     RoleAlltoB,
//     recv_mpst_c_all_to_b,
//     RoleC,
//     SessionMpstFive,
//     5,
//     2
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_c_to_d,
//     OfferMpstMultiFive,
//     RoleAlltoD,
//     recv_mpst_c_all_to_d,
//     RoleC,
//     SessionMpstFive,
//     5,
//     3
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_c_to_e,
//     OfferMpstMultiFive,
//     RoleAlltoE,
//     recv_mpst_c_all_to_e,
//     RoleC,
//     SessionMpstFive,
//     5,
//     4
// );
// // D
// create_offer_mpst_session_multi!(
//     offer_mpst_session_d_to_a,
//     OfferMpstMultiFive,
//     RoleAlltoA,
//     recv_mpst_d_all_to_a,
//     RoleD,
//     SessionMpstFive,
//     5,
//     1
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_d_to_b,
//     OfferMpstMultiFive,
//     RoleAlltoB,
//     recv_mpst_d_all_to_b,
//     RoleD,
//     SessionMpstFive,
//     5,
//     2
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_d_to_c,
//     OfferMpstMultiFive,
//     RoleAlltoC,
//     recv_mpst_d_all_to_c,
//     RoleD,
//     SessionMpstFive,
//     5,
//     3
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_d_to_e,
//     OfferMpstMultiFive,
//     RoleAlltoE,
//     recv_mpst_d_all_to_e,
//     RoleD,
//     SessionMpstFive,
//     5,
//     4
// );
// // E
// create_offer_mpst_session_multi!(
//     offer_mpst_session_e_to_a,
//     OfferMpstMultiFive,
//     RoleAlltoA,
//     recv_mpst_e_all_to_a,
//     RoleE,
//     SessionMpstFive,
//     5,
//     1
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_e_to_b,
//     OfferMpstMultiFive,
//     RoleAlltoB,
//     recv_mpst_e_all_to_b,
//     RoleE,
//     SessionMpstFive,
//     5,
//     2
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_e_to_c,
//     OfferMpstMultiFive,
//     RoleAlltoC,
//     recv_mpst_e_all_to_c,
//     RoleE,
//     SessionMpstFive,
//     5,
//     3
// );
// create_offer_mpst_session_multi!(
//     offer_mpst_session_e_to_d,
//     OfferMpstMultiFive,
//     RoleAlltoD,
//     recv_mpst_e_all_to_d,
//     RoleE,
//     SessionMpstFive,
//     5,
//     4
// );

// // Create choose functions
// // A
// create_choose_mpst_session_multi_both!(
//     choose_left_mpst_session_a_to_all,
//     choose_right_mpst_session_a_to_all,
//     ChooseMpstFive,
//     RoleAtoAll,
//     next_a_to_all,
//     RoleA,
//     SessionMpstFive,
//     5
// );
// // B
// create_choose_mpst_session_multi_both!(
//     choose_left_mpst_session_b_to_all,
//     choose_right_mpst_session_b_to_all,
//     ChooseMpstFive,
//     RoleBtoAll,
//     next_b_to_all,
//     RoleB,
//     SessionMpstFive,
//     5
// );
// // C
// create_choose_mpst_session_multi_both!(
//     choose_left_mpst_session_c_to_all,
//     choose_right_mpst_session_c_to_all,
//     ChooseMpstFive,
//     RoleCtoAll,
//     next_c_to_all,
//     RoleC,
//     SessionMpstFive,
//     5
// );
// // D
// create_choose_mpst_session_multi_both!(
//     choose_left_mpst_session_d_to_all,
//     choose_right_mpst_session_d_to_all,
//     ChooseMpstFive,
//     RoleDtoAll,
//     next_d_to_all,
//     RoleD,
//     SessionMpstFive,
//     5
// );
// // E
// create_choose_mpst_session_multi_both!(
//     choose_left_mpst_session_e_to_all,
//     choose_right_mpst_session_e_to_all,
//     ChooseMpstFive,
//     RoleEtoAll,
//     next_e_to_all,
//     RoleE,
//     SessionMpstFive,
//     5
// );

// // Create close function
// close_mpst!(close_mpst_multi, SessionMpstFive, 5);

// // Create fork function
// bundle_fork_multi!(fork_mpst, fork_simple, SessionMpstFive, 5);

// // Names
// type NameA = RoleA<RoleEnd>;
// type NameB = RoleB<RoleEnd>;
// type NameC = RoleC<RoleEnd>;
// type NameD = RoleD<RoleEnd>;
// type NameE = RoleE<RoleEnd>;

// // Queues

// // Types
// // Offer
// // A
// type OfferA = OfferMpstMultiFive<End, End, End, End, End, End, End, End, RoleEnd, RoleEnd, NameA>;
// // B
// type OfferB = OfferMpstMultiFive<End, End, End, End, End, End, End, End, RoleEnd, RoleEnd, NameB>;
// // C
// type OfferC = OfferMpstMultiFive<End, End, End, End, End, End, End, End, RoleEnd, RoleEnd, NameC>;
// // D
// type OfferD = OfferMpstMultiFive<End, End, End, End, End, End, End, End, RoleEnd, RoleEnd, NameD>;
// // E
// type OfferE = OfferMpstMultiFive<End, End, End, End, End, End, End, End, RoleEnd, RoleEnd, NameE>;

// // Binary
// // A
// type AtoB<N> = Send<N, Recv<N, End>>;
// type AtoC<N> = Send<N, Recv<N, End>>;
// type AtoD<N> = Send<N, Recv<N, End>>;
// type AtoE<N> = Send<N, Recv<N, End>>;
// // B
// type BtoA<N> = Recv<N, Send<N, End>>;
// type BtoC<N> = Send<N, Recv<N, End>>;
// type BtoD<N> = Send<N, Recv<N, End>>;
// type BtoE<N> = Send<N, Recv<N, End>>;
// // C
// type CtoA<N> = Recv<N, Send<N, End>>;
// type CtoB<N> = Recv<N, Send<N, End>>;
// type CtoD<N> = Send<N, Recv<N, End>>;
// type CtoE<N> = Send<N, Recv<N, End>>;
// // D
// type DtoA<N> = Recv<N, Send<N, End>>;
// type DtoB<N> = Recv<N, Send<N, End>>;
// type DtoC<N> = Recv<N, Send<N, End>>;
// type DtoE<N> = Send<N, Recv<N, End>>;
// // E
// type EtoA<N> = Recv<N, Send<N, End>>;
// type EtoB<N> = Recv<N, Send<N, End>>;
// type EtoC<N> = Recv<N, Send<N, End>>;
// type EtoD<N> = Recv<N, Send<N, End>>;
