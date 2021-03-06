////////////////////////////////////////////
/// CHOICE

#[macro_export]
macro_rules! create_choose_type_multi {
    ($type_name: ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions {
            type $type_name<
                #(S#N:0,)2:0 R0, R1, N0
            > = mpstthree::binary::Send<
                either::Either<
                    <
                        $sessionmpst_name<#(S#N:0,)0:0 R0, N0
                    > as mpstthree::binary::Session>::Dual,
                    <
                        $sessionmpst_name<#(S#N:0,)3:0 R1, N0
                    > as mpstthree::binary::Session>::Dual
                    >,
                End
            >;
        });
    }
}

#[macro_export]
macro_rules! create_choose_mpst_session_multi_left {
    ($func_name:ident, $type_name: ident, $role:ident, $recv_func:ident, $name:ident, $sessionmpst_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions {
            fn $func_name<'a, #(S#N:0,)10:0 #(R#N:0,)11:0>(
                s: $sessionmpst_name<
                    #( // i in 1..K
                        $type_name<
                            ~( // j in 0..K
                                <S~N:2 as mpstthree::binary::Session>::Dual,
                            )(
                                S~N:2, // S(i + j) (with Dual if needed)
                            )0*
                            ~( // j in 0..K
                                <S~N:3 as mpstthree::binary::Session>::Dual,
                            )(
                                S~N:3, // S(diff * (diff + 1) / 2 + K + i + j) (with Dual if needed)
                            )0*
                            ~( // j in  0..3
                                R~N:4, // R(3 * (i - 1) + 1 + j)
                                // Side note: we lose the checking for the right order for the name on R(3 * (i - 1) + 3) → RoleADual<RoleEnd>
                            )()3*
                        >,
                    )0:0

                    $role<
                        #(
                            R#N:0,
                        )13:0
                    >,

                    $name<mpstthree::role::end::RoleEnd>,
                >,
            )
            -> $sessionmpst_name<
                #( // K-1 + i in (K-1..0)
                    <S#N:8 as mpstthree::binary::Session>::Dual, // S(i) or  S(i + diff * (diff + 1))
                )0:0
                R^N:0, // R(3K-2) or R(3K-1)
                $name<mpstthree::role::end::RoleEnd>
            >
            where
                #( // i in 1..(diff * (diff + 1) + 1)
                    S#N:0: mpstthree::binary::Session + 'a, // S(i)
                )10:0
                #( // i in 1..(3 * K)
                    R#N:0: mpstthree::role::Role + 'a, // R(i)
                )11:0
            {
                #( // i in 1..(diff * (diff + 1))
                    let (channel_#N:3, channel_#N:4) = S#N:0::new(); // channel_(get from matrix), channel_(opposite get from matrix) = S(i)
                )4:0

                #( // i in 1..K
                    let (_, stack_#N:0) = R#N:5::new();
                )0:0
                let (stack_^N:2, _) = R^N:0::new();

                #( // i in 1..K
                    let (name_#N:0, _) = <R#N:6 as mpstthree::role::Role>::Dual::new();
                )0:0
                let (name_^N:2, _) = $name::<mpstthree::role::end::RoleEnd>::new();

                #( // i in 1..K
                    let choice_#N:0 = $sessionmpst_name {
                            ~(
                                session#N:1 : channel_~N:5,
                            )(
                                session#N:1 : channel_~N:5,
                            )0*
                            stack: stack_#N:0,
                            name: name_#N:0,
                        };
                )0:0

                #( // i in 1..K
                    let new_session_#N:0 = mpstthree::binary::send(either::Either::Left(choice_#N:0), s.session#N:0);
                )0:0
                let (_, new_queue) = $recv_func(s.stack);

                let s = $sessionmpst_name {
                    #(
                        session#N:0: new_session_#N:0,
                    )0:0
                    stack: new_queue,
                    name: s.name,
                };

                mpstthree::binary::cancel(s);

                $sessionmpst_name {
                    #(
                        session#N:0: channel_#N:7 ,
                    )0:0
                    stack: stack_^N:2,
                    name: name_^N:2,
                }
            }
        });
    }
}
#[macro_export]
macro_rules! create_choose_mpst_session_multi_right {
    ($func_name:ident, $type_name: ident, $role:ident, $recv_func:ident, $name:ident, $sessionmpst_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions {
            fn $func_name<'a, #(S#N:0,)10:0 #(R#N:0,)11:0>(
                s: $sessionmpst_name<
                    #( // i in 1..K
                        $type_name<
                            ~( // j in 0..K
                                <S~N:2 as mpstthree::binary::Session>::Dual,
                            )(
                                S~N:2, // S(i + j) (with Dual if needed)
                            )0*
                            ~( // j in 0..K
                                <S~N:3 as mpstthree::binary::Session>::Dual,
                            )(
                                S~N:3, // S(diff * (diff + 1) / 2 + K + i + j) (with Dual if needed)
                            )0*
                            ~( // j in  0..3
                                R~N:4, // R(3 * (i - 1) + 1 + j)
                                // Side note: we lose the checking for the right order for the name on R(3 * (i - 1) + 3) → RoleADual<RoleEnd>
                            )()3*
                        >,
                    )0:0

                    $role<
                        #(
                            R#N:0,
                        )13:0
                    >,

                    $name<mpstthree::role::end::RoleEnd>,
                >,
            )
            -> $sessionmpst_name<
                #( // K-1 + i in (K-1..0)
                    <S#N:9 as mpstthree::binary::Session>::Dual, // S(i) or  S(i + diff * (diff + 1))
                )0:0
                R^N:1, // R(3K-2) or R(3K-1)
                $name<mpstthree::role::end::RoleEnd>
            >
            where
                #( // i in 1..(diff * (diff + 1) + 1)
                    S#N:0: mpstthree::binary::Session + 'a, // S(i)
                )10:0
                #( // i in 1..(3 * K)
                    R#N:0: mpstthree::role::Role + 'a, // R(i)
                )11:0
            {
                #( // i in 1..(diff * (diff + 1))
                    let (channel_#N:3, channel_#N:4) = S#N:10::new(); // channel_(get from matrix), channel_(opposite get from matrix) = S(i)
                )4:0

                #( // i in 1..K
                    let (_, stack_#N:0) = R#N:11::new();
                )0:0
                let (stack_^N:2, _) = R^N:1::new();

                #( // i in 1..K
                    let (name_#N:0, _) = <R#N:6 as mpstthree::role::Role>::Dual::new();
                )0:0
                let (name_^N:2, _) = $name::<mpstthree::role::end::RoleEnd>::new();

                #( // i in 1..K
                    let choice_#N:0 = $sessionmpst_name {
                            ~(
                                session#N:1 : channel_~N:5,
                            )(
                                session#N:1 : channel_~N:5,
                            )0*
                            stack: stack_#N:0,
                            name: name_#N:0,
                        };
                )0:0

                #( // i in 1..K
                    let new_session_#N:0 = mpstthree::binary::send(either::Either::Right(choice_#N:0), s.session#N:0);
                )0:0
                let (_, new_queue) = $recv_func(s.stack);

                let s = $sessionmpst_name {
                    #(
                        session#N:0: new_session_#N:0,
                    )0:0
                    stack: new_queue,
                    name: s.name,
                };

                mpstthree::binary::cancel(s);

                $sessionmpst_name {
                    #(
                        session#N:0: channel_#N:7 ,
                    )0:0
                    stack: stack_^N:2,
                    name: name_^N:2,
                }
            }
        });
    }
}

/// Choose between many different sessions wrapped in an `enum`
/// TODO
#[macro_export]
macro_rules! choose_mpst_X_to_all {
    ($session:expr, $($fn_send:ident,)+ => $($label:path,)+ => $($receiver:ident,)+ => $sender:ident, $sessionmpst_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! 1 : ($($fn_send$args,)+) : ($($label,)+) : ($($receiver,)+) {{

            #(
                let (channel_#N:3, channel_#N:4) = <_ as mpstthree::binary::Session>::new();
            )4:0

            #(
                let (stack_#N:0, _) = <_ as mpstthree::role::Role>::new();
            )15:0

            #(
                let (name_#N:0, _) = useless#N:16::<mpstthree::role::end::RoleEnd>::new();
            )0:0

            let (name_^N:2, _) = <$sender<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

            %(
                let s = useless#N:14(
                    useless#N:15($sessionmpst_name {
                        ~(
                            session#N:1 : channel_~N:5,
                        )(
                            session#N:1 : channel_~N:5,
                        )0*
                        stack: stack_#N:0,
                        name: name_#N:0,
                    }),
                    s,
                );
            )(
                let s = useless#N:14(
                    useless#N:15($sessionmpst_name {
                        ~(
                            session#N:1 : channel_~N:5,
                        )(
                            session#N:1 : channel_~N:5,
                        )0*
                        stack: stack_#N:0,
                        name: name_#N:0,
                    }),
                    $session,
                );
            )0*

            mpstthree::binary::cancel(s);

            $sessionmpst_name {
                #(
                    session#N:0: channel_#N:7 ,
                )0:0
                stack: stack_^N:2,
                name: name_^N:2,
            }
            }
        });
    }
}
