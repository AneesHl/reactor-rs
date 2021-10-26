/*
 * Copyright (c) 2021, TU Dresden.
 *
 * Redistribution and use in source and binary forms, with or without modification,
 * are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
 * THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#![allow(unused, non_snake_case, non_camel_case_types)]
#[macro_use]
extern crate reactor_rt;


use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use ::reactor_rt::{ Instant, Duration};
use ::reactor_rt::Offset::{After, Asap};

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main, black_box};

use reactor_rt::*;

/*
The ping/pong game from Savina benchmarks. This can be compared
to the C implementation (see results.md).

See original at https://github.com/icyphy/lingua-franca/blob/f5868bec199e02f784393f32b594be5df935e2ee/benchmark/C/Savina/PingPong.lf


 */

criterion_group!(benches, reactor_main);
criterion_main!(benches);

fn reactor_main(c: &mut Criterion) {
    fn init_logger() {
        env_logger::Builder::from_env(env_logger::Env::default())
            .format_target(false)
            .init();
    }
    init_logger();

    let mut group = c.benchmark_group("savina_pong");
    for num_pongs in [1000, 10_000, 100_000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_pongs),
            num_pongs,
            |b, &size| {
                b.iter(|| {
                    let timeout = Some(Duration::from_secs(5));
                    launch(1, size, timeout);
                });
            },
        );
    }
    group.finish();
}

fn launch(numIterations: u32, count: u32, timeout: Option<Duration>)  {

    let options = SchedulerOptions {
        timeout: None,
        keep_alive: false
    };
    let main_args = reactors::SavinaPongParams::new(count);

    SyncScheduler::run_main::<reactors::SavinaPongAdapter>(options, main_args);
}

//-------------------//
//---- REACTORS -----//
//-------------------//
mod reactors {
    pub use self::pong::PongAdapter;
    pub use self::pong::PongParams;
    pub use self::ping::PingAdapter;
    pub use self::ping::PingParams;
    pub use self::savina_pong::SavinaPongAdapter;
    pub use self::savina_pong::SavinaPongParams;
    //--------------------------------------------//
    //------------ Pong -------//
    //-------------------//
    mod pong {
        //-- Generated by LFC @ 2021/10/26 19:59:23 --//
        #![allow(unused)]

        use ::reactor_rt::prelude::*;



        /// Generated from /home/clem/Documents/LF/reactor-rust/benches/SavinaPong.lf:25
        ///
        /// --- reactor Pong(expected: u32(1000000)) { ... }
        pub struct Pong {
            pub __phantom: std::marker::PhantomData<()>,
            expected: u32,
            count: u32,
        }

        #[warn(unused)]
        impl Pong {

            // --- reaction(receive) -> send {= ... =}
            fn react_0(&mut self,
                       #[allow(unused)] ctx: &mut ::reactor_rt::ReactionCtx,
                       receive: &::reactor_rt::ReadablePort<u32>,
                       #[allow(unused_mut)] mut send: ::reactor_rt::WritablePort<u32>,) {
                self.count += 1;
                ctx.set(send, ctx.get(receive).unwrap());
            }

            // --- reaction(shutdown) {= ... =}
            fn react_1(&mut self,
                       #[allow(unused)] ctx: &mut ::reactor_rt::ReactionCtx,
            ) {
                assert_eq!(self.count, self.expected);
            }

        }

        /// Parameters for the construction of a [Pong]
        pub struct PongParams {
            __phantom: std::marker::PhantomData<()>,
            expected: u32,
        }

        impl PongParams {
            pub fn new(
                expected: u32,
            ) -> Self {
                Self { __phantom: std::marker::PhantomData, expected, }
            }
        }

        //------------------------//


        pub struct PongAdapter {
            __id: ::reactor_rt::ReactorId,
            __impl: Pong,
            pub __send: ::reactor_rt::Port<u32>,
            pub __receive: ::reactor_rt::Port<u32>,
        }

        impl PongAdapter {
            #[inline]
            fn user_assemble(__assembler: &mut ::reactor_rt::assembly::ComponentCreator<Self>,
                             __id: ::reactor_rt::ReactorId,
                             __params: PongParams) -> ::reactor_rt::assembly::AssemblyResult<Self> {
                let PongParams {  __phantom, expected, } = __params;

                let __impl = {
                    // declare them all here so that they are visible to the initializers of state vars declared later
                    let expected = expected;
                    let count = 0;

                    Pong {
                        __phantom: std::marker::PhantomData,
                        expected,
                        count,
                    }
                };

                Ok(Self {
                    __id,
                    __impl,
                    __send: __assembler.new_port::<u32>("send", false),
                    __receive: __assembler.new_port::<u32>("receive", true),
                })
            }
        }

        impl ::reactor_rt::assembly::ReactorInitializer for PongAdapter {
            type Wrapped = Pong;
            type Params = PongParams;
            const MAX_REACTION_ID: ::reactor_rt::LocalReactionId = ::reactor_rt::LocalReactionId::new(3 - 1);

            fn assemble(__params: Self::Params, __ctx: ::reactor_rt::assembly::AssemblyCtx<Self>) -> ::reactor_rt::assembly::AssemblyResult<Self> {
                use ::reactor_rt::assembly::TriggerLike;

                let PongParams {  __phantom, expected, } = __params;
                let (_, __self) =
                    __ctx.do_assembly(
                        |cc, id| Self::user_assemble(cc, id, PongParams {  __phantom, expected, }),
                        // number of non-synthetic reactions
                        2,
                        // reaction debug labels
                        [None, None],
                        // dependency declarations
                        |__assembler, __self, [react_0, react_1]| {
                            #[allow(unused)]
                            use reactor_rt::unsafe_iter_bank;
                            // --- reaction(receive) -> send {= ... =}
                            __assembler.declare_triggers(__self.__receive.get_id(), react_0)?;
                            __assembler.effects_port(react_0, &__self.__send)?;
                            // --- reaction(shutdown) {= ... =}
                            __assembler.declare_triggers(::reactor_rt::assembly::TriggerId::SHUTDOWN, react_1)?;
                            // Declare connections

                            // Declare port references

                            Ok(())
                        }
                    )
                        ?;

                Ok(__self)
            }
        }


        impl ::reactor_rt::ReactorBehavior for PongAdapter {

            #[inline]
            fn id(&self) -> ::reactor_rt::ReactorId {
                self.__id
            }

            fn react_erased(&mut self, ctx: &mut ::reactor_rt::ReactionCtx, rid: ::reactor_rt::LocalReactionId) {
                match rid.raw() {
                    0 => self.__impl.react_0(ctx, &::reactor_rt::ReadablePort::new(&self.__receive), ::reactor_rt::WritablePort::new(&mut self.__send),),
                    1 => self.__impl.react_1(ctx),

                    _ => panic!("Invalid reaction ID: {} should be < {}", rid, <Self as ::reactor_rt::assembly::ReactorInitializer>::MAX_REACTION_ID)
                }
            }

            fn cleanup_tag(&mut self, ctx: &::reactor_rt::CleanupCtx) {
                ctx.cleanup_port(&mut self.__send);
                ctx.cleanup_port(&mut self.__receive);
            }

        }
    }


    //--------------------------------------------//
    //------------ Ping -------//
    //-------------------//
    mod ping {
        //-- Generated by LFC @ 2021/10/26 19:59:23 --//
        #![allow(unused)]

        use ::reactor_rt::prelude::*;



        /// Generated from /home/clem/Documents/LF/reactor-rust/benches/SavinaPong.lf:5
        ///
        /// --- reactor Ping(count: u32(1000000)) { ... }
        pub struct Ping {
            pub __phantom: std::marker::PhantomData<()>,
            pingsLeft: u32,
        }

        #[warn(unused)]
        impl Ping {

            // --- reaction(startup, serve) -> send {= ... =}
            fn react_0(&mut self,
                       #[allow(unused)] ctx: &mut ::reactor_rt::ReactionCtx,
                       #[allow(unused)] serve: &::reactor_rt::LogicalAction<()>,
                       #[allow(unused_mut)] mut send: ::reactor_rt::WritablePort<u32>,) {
                ctx.set(send, self.pingsLeft);
                self.pingsLeft -= 1;
            }

            // --- reaction (receive) -> serve {= ... =}
            fn react_1(&mut self,
                       #[allow(unused)] ctx: &mut ::reactor_rt::ReactionCtx,
                       receive: &::reactor_rt::ReadablePort<u32>,
                       #[allow(unused_mut)] mut serve: &mut ::reactor_rt::LogicalAction<()>,) {
                if self.pingsLeft > 0 {
                    ctx.schedule(serve, Asap);
                } else {
                    ctx.request_stop(Asap);
                }
            }

        }

        /// Parameters for the construction of a [Ping]
        pub struct PingParams {
            __phantom: std::marker::PhantomData<()>,
            count: u32,
        }

        impl PingParams {
            pub fn new(
                count: u32,
            ) -> Self {
                Self { __phantom: std::marker::PhantomData, count, }
            }
        }

        //------------------------//


        pub struct PingAdapter {
            __id: ::reactor_rt::ReactorId,
            __impl: Ping,
            pub __send: ::reactor_rt::Port<u32>,
            pub __receive: ::reactor_rt::Port<u32>,
            __serve: ::reactor_rt::LogicalAction<()>,
        }

        impl PingAdapter {
            #[inline]
            fn user_assemble(__assembler: &mut ::reactor_rt::assembly::ComponentCreator<Self>,
                             __id: ::reactor_rt::ReactorId,
                             __params: PingParams) -> ::reactor_rt::assembly::AssemblyResult<Self> {
                let PingParams {  __phantom, count, } = __params;

                let __impl = {
                    // declare them all here so that they are visible to the initializers of state vars declared later
                    let pingsLeft = count;

                    Ping {
                        __phantom: std::marker::PhantomData,
                        pingsLeft,
                    }
                };

                Ok(Self {
                    __id,
                    __impl,
                    __send: __assembler.new_port::<u32>("send", false),
                    __receive: __assembler.new_port::<u32>("receive", true),
                    __serve: __assembler.new_logical_action::<()>("serve", None),
                })
            }
        }

        impl ::reactor_rt::assembly::ReactorInitializer for PingAdapter {
            type Wrapped = Ping;
            type Params = PingParams;
            const MAX_REACTION_ID: ::reactor_rt::LocalReactionId = ::reactor_rt::LocalReactionId::new(3 - 1);

            fn assemble(__params: Self::Params, __ctx: ::reactor_rt::assembly::AssemblyCtx<Self>) -> ::reactor_rt::assembly::AssemblyResult<Self> {
                use ::reactor_rt::assembly::TriggerLike;

                let PingParams {  __phantom, count, } = __params;
                let (_, __self) =
                    __ctx.do_assembly(
                        |cc, id| Self::user_assemble(cc, id, PingParams {  __phantom, count, }),
                        // number of non-synthetic reactions
                        2,
                        // reaction debug labels
                        [None, None],
                        // dependency declarations
                        |__assembler, __self, [react_0, react_1]| {
                            #[allow(unused)]
                            use reactor_rt::unsafe_iter_bank;
                            // --- reaction(startup, serve) -> send {= ... =}
                            __assembler.declare_triggers(__self.__serve.get_id(), react_0)?;
                            __assembler.declare_triggers(::reactor_rt::assembly::TriggerId::STARTUP, react_0)?;
                            __assembler.effects_port(react_0, &__self.__send)?;
                            // --- reaction (receive) -> serve {= ... =}
                            __assembler.declare_triggers(__self.__receive.get_id(), react_1)?;
                            // Declare connections

                            // Declare port references

                            Ok(())
                        }
                    )
                        ?;

                Ok(__self)
            }
        }


        impl ::reactor_rt::ReactorBehavior for PingAdapter {

            #[inline]
            fn id(&self) -> ::reactor_rt::ReactorId {
                self.__id
            }

            fn react_erased(&mut self, ctx: &mut ::reactor_rt::ReactionCtx, rid: ::reactor_rt::LocalReactionId) {
                match rid.raw() {
                    0 => self.__impl.react_0(ctx, &self.__serve, ::reactor_rt::WritablePort::new(&mut self.__send),),
                    1 => self.__impl.react_1(ctx, &::reactor_rt::ReadablePort::new(&self.__receive), &mut self.__serve,),

                    _ => panic!("Invalid reaction ID: {} should be < {}", rid, <Self as ::reactor_rt::assembly::ReactorInitializer>::MAX_REACTION_ID)
                }
            }

            fn cleanup_tag(&mut self, ctx: &::reactor_rt::CleanupCtx) {
                ctx.cleanup_port(&mut self.__send);
                ctx.cleanup_port(&mut self.__receive);
                ctx.cleanup_logical_action(&mut self.__serve);
            }

        }
    }


    //--------------------------------------------//
    //------------ SavinaPong -------//
    //-------------------//
    mod savina_pong {
        //-- Generated by LFC @ 2021/10/26 19:59:23 --//
        #![allow(unused)]

        use ::reactor_rt::prelude::*;



        /// Generated from /home/clem/Documents/LF/reactor-rust/benches/SavinaPong.lf:42
        ///
        /// --- main reactor SavinaPong(count: u32(1000000)) { ... }
        pub struct SavinaPong {
            pub __phantom: std::marker::PhantomData<()>,

        }

        #[warn(unused)]
        impl SavinaPong {



        }

        /// Parameters for the construction of a [SavinaPong]
        pub struct SavinaPongParams {
            __phantom: std::marker::PhantomData<()>,
            count: u32,
        }

        impl SavinaPongParams {
            pub fn new(
                count: u32,
            ) -> Self {
                Self { __phantom: std::marker::PhantomData, count, }
            }
        }

        //------------------------//


        pub struct SavinaPongAdapter {
            __id: ::reactor_rt::ReactorId,
            __impl: SavinaPong,

        }

        impl SavinaPongAdapter {
            #[inline]
            fn user_assemble(__assembler: &mut ::reactor_rt::assembly::ComponentCreator<Self>,
                             __id: ::reactor_rt::ReactorId,
                             __params: SavinaPongParams) -> ::reactor_rt::assembly::AssemblyResult<Self> {
                let SavinaPongParams {  __phantom, count, } = __params;

                let __impl = {
                    // declare them all here so that they are visible to the initializers of state vars declared later


                    SavinaPong {
                        __phantom: std::marker::PhantomData,

                    }
                };

                Ok(Self {
                    __id,
                    __impl,

                })
            }
        }

        impl ::reactor_rt::assembly::ReactorInitializer for SavinaPongAdapter {
            type Wrapped = SavinaPong;
            type Params = SavinaPongParams;
            const MAX_REACTION_ID: ::reactor_rt::LocalReactionId = ::reactor_rt::LocalReactionId::new(1 - 1);

            fn assemble(__params: Self::Params, __ctx: ::reactor_rt::assembly::AssemblyCtx<Self>) -> ::reactor_rt::assembly::AssemblyResult<Self> {
                use ::reactor_rt::assembly::TriggerLike;

                let SavinaPongParams {  __phantom, count, } = __params;
                let (_, __self) =
                    __ctx.with_child::<super::PingAdapter, _>("ping", super::PingParams::new(count,), |mut __ctx, ping| {
                        __ctx.with_child::<super::PongAdapter, _>("pong", super::PongParams::new(count,), |mut __ctx, pong| {
                            __ctx.do_assembly(
                                |cc, id| Self::user_assemble(cc, id, SavinaPongParams {  __phantom, count, }),
                                // number of non-synthetic reactions
                                0,
                                // reaction debug labels
                                [],
                                // dependency declarations
                                |__assembler, __self, []| {
                                    #[allow(unused)]
                                    use reactor_rt::unsafe_iter_bank;

                                    // Declare connections
                                    { // --- ping.send -> pong.receive;
                                        let up = std::iter::once(&mut ping.__send);
                                        let down = std::iter::once(&mut pong.__receive);
                                        __assembler.bind_ports_zip(up, down)?;
                                    }
                                    { // --- pong.send -> ping.receive;
                                        let up = std::iter::once(&mut pong.__send);
                                        let down = std::iter::once(&mut ping.__receive);
                                        __assembler.bind_ports_zip(up, down)?;
                                    }
                                    // Declare port references

                                    Ok(())
                                }
                            )
                        })})?;

                Ok(__self)
            }
        }


        impl ::reactor_rt::ReactorBehavior for SavinaPongAdapter {

            #[inline]
            fn id(&self) -> ::reactor_rt::ReactorId {
                self.__id
            }

            fn react_erased(&mut self, ctx: &mut ::reactor_rt::ReactionCtx, rid: ::reactor_rt::LocalReactionId) {
                match rid.raw() {


                    _ => panic!("Invalid reaction ID: {} should be < {}", rid, <Self as ::reactor_rt::assembly::ReactorInitializer>::MAX_REACTION_ID)
                }
            }

            fn cleanup_tag(&mut self, ctx: &::reactor_rt::CleanupCtx) {

            }

        }
    }



}
