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

#[cfg(test)]
pub mod test;

pub use self::actions::*;
pub use self::timers::*;
pub use self::reactions::*;
pub use self::ports::*;

pub use self::scheduler::*;
pub use self::time::*;
pub use self::util::*;
pub use self::ids::*;

// reexport those to complement our LogicalInstant
pub use std::time::Instant as PhysicalInstant;
pub use std::time::Duration;


mod scheduler;
mod ports;
mod actions;
mod time;
mod timers;
mod reactions;
mod util;
mod ids;

#[macro_use]
extern crate log;

#[macro_use]
extern crate index_vec;


/// Wrapper around the user struct for safe dispatch.
///
/// Fields are
/// 1. the user struct,
/// 2. ctor parameters of the reactor, and
/// 3. every logical action and port declared by the reactor.
///
pub trait ReactorDispatcher: ErasedReactorDispatcher {
    /// Type of the user struct
    type Wrapped;
    /// Type of the construction parameters
    type Params;

    /// Exclusive maximum value of the `local_rid` parameter of [ErasedReactorDispatcher.react_erased].
    const MAX_REACTION_ID: LocalReactionId;

    /// Assemble the user reactor, ie produce components with
    /// uninitialized dependencies & make state variables assume
    /// their default values, or else, a value taken from the params.
    fn assemble(args: Self::Params, assembler: &mut AssemblyCtx)
                -> Self where Self: Sized;

}

pub trait ErasedReactorDispatcher {

    /// The unique ID of this reactor. This is given by the
    /// framework upon construction.
    fn id(&self) -> ReactorId;

    /// Execute a single user-written reaction.
    /// Dispatches on the reaction id, and unpacks parameters,
    /// which are the reactor components declared as fields of
    /// this struct.
    ///
    /// It must always be the case that `local_rid < Self::MAX_REACTION_ID`.
    fn react_erased(&mut self, ctx: &mut LogicalCtx, local_rid: LocalReactionId);

    /// Acknowledge that the given tag is done executing and
    /// free resources if need be.
    /// TODO this is not implemented
    fn cleanup_tag(&mut self, ctx: LogicalCtx);

    /// Enqueue the startup reactions of this reactor into
    /// the parameter. Timers are also started at this point,
    /// meaning, their first triggering is scheduled.
    ///
    /// During startup of the program, this method is called
    /// on every reactor of the program to build the first
    /// reaction wave. This is then executed to completion,
    /// producing new events which drive the program further.
    ///
    fn enqueue_startup(&self, ctx: &mut StartupCtx);

    /// Enqueue the shutdown reactions of this reactor.
    /// See [enqueue_startup].
    fn enqueue_shutdown(&self, ctx: &mut StartupCtx);
}

#[macro_export]
#[doc(hidden)]
macro_rules! new_reaction {
    ($reactorid:ident, $_rstate:ident, $id:literal) => {{
        $crate::GlobalReactionId::new($reactorid, $id)
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! reschedule_self_timer {
    ($reactorid:ident, $timerid:ident, $_rstate:ident, $_rpriority:literal) => {{
        let mut mystate = $_rstate.clone();
        let schedule_myself = move |ctx: &mut $crate::LogicalCtx| {
            let me = mystate.lock().unwrap();
            ctx.reschedule(&me.$timerid); // this doesn't reschedule aperiodic timers
        };
        ::std::sync::Arc::new($crate::ReactionInvoker::new_from_closure($reactorid, $_rpriority, schedule_myself))
    }};
}
