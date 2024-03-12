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

//! The runtime library for reactor programs generated by LFC,
//! the [Lingua Franca](https://github.com/lf-lang/lingua-franca) compiler.
//!
//! Most of this crate consists of types that are manipulated
//! only by the generated code. User-written Rust code mostly
//! interacts with the type [ReactionCtx], which is the entry
//! point for user documentation.
//!
//! Crate-level features include:
//! - `parallel-runtime`: use Rayon to execute reactions in parallel
//! when possible. This is not yet the default. For some applications,
//! where there is no data parallelism, this may harm performance
//! (as well as pull in unneeded dependencies) and should be off.
//! - `wide-ids`: Enables 64-bit wide reaction ids on 64-bit
//! architectures. This may reduce performance, but allows for
//! 2^32 reactor instances compared to the default of 2^16,
//! which may feel a bit tight for some applications. On machines
//! with a pointer-width of less than 64 bits, ID types are
//! always 32 bits. The feature also widens trigger ids to 64 bits
//! if possible, which enables 2^64 individual trigger components
//! (ports, actions, etc.) instead of 2^32.
//! - `vec-id-sets`: Change the implementation of reaction sets
//! to be a sorted vector instead of a hash set. This has a positive
//! performance impact, as reaction sets are typically very small.
//! More testing is required to determine pathological cases.
//! This is a default feature.
//! - `no-unsafe`: disable optimisations that use unsafe code in this runtime.
//! Just provided for comparison, should probably be removed (unsafe code is fine).

// #![deny(unused_crate_dependencies)]
#![deny(unused_extern_crates)]
// #![warn(unreachable_pub)]
#![warn(unused_lifetimes)]
#![warn(single_use_lifetimes)]
#![warn(explicit_outlives_requirements)]

#[macro_use]
extern crate array_macro;
#[cfg(test)]
#[allow(unused)]
#[macro_use]
extern crate assert_matches;
#[macro_use]
extern crate index_vec;
#[macro_use]
extern crate log;
#[cfg(feature = "parallel-runtime")]
extern crate rayon;
#[macro_use]
extern crate smallvec;
#[macro_use]
extern crate static_assertions;
#[macro_use]
extern crate cfg_if;

pub use std::time::{Duration, Instant};

pub(crate) use scheduler::debug::*;

pub use self::actions::*;
pub use self::ids::*;
pub use self::ports::*;
pub use self::scheduler::*;
pub use self::time::*;
pub use self::timers::*;
pub use self::triggers::ReactionTrigger;
pub use self::util::*;

#[cfg(test)]
pub mod test;

mod actions;
mod ids;
mod ports;
mod scheduler;
mod time;
mod timers;
mod triggers;
mod util;

pub mod assembly;

/// The prelude that is imported at the top of reactor files
/// generated by LFC.
pub mod prelude {
    pub use crate::Offset::*;
    pub use crate::{
        after, assert_tag_is, delay, tag, AsyncCtx, Duration, EventTag, Instant, LogicalAction, Multiport, PhysicalActionRef,
        Port, ReactionCtx, Timer,
    };

    /// Alias for the unit type, so that it can be written without quotes in LF.
    /// Otherwise it needs to be written `{= () =}`.
    /// It is not camel-case as it is actually a primitive type.
    #[allow(non_camel_case_types)]
    pub type unit = ();
}

/// The trait used by the framework to interact with the reactor
/// during runtime.
///
/// Importantly, it's object-safe and has no type parameters
/// or associated types. This allows us to wrap it into a
/// `Box<dyn ReactorBehavior>`.
pub trait ReactorBehavior {
    /// The unique ID of this reactor. This is given by the
    /// framework upon construction.
    fn id(&self) -> ReactorId;

    /// Execute a single user-written reaction.
    /// Dispatches on the reaction id, and unpacks parameters,
    /// which are the reactor components declared as fields of
    /// this struct.
    ///
    /// It must always be the case that `local_rid < Self::MAX_REACTION_ID`,
    /// where `Self::MAX_REACTION_ID` is defined by the [assembly::ReactorInitializer],
    /// because of object safety.
    fn react(&mut self, ctx: &mut ReactionCtx, local_rid: LocalReactionId);

    /// Acknowledge that the given tag is done executing and
    /// free resources if need be.
    fn cleanup_tag(&mut self, ctx: &CleanupCtx);
}
assert_obj_safe!(ReactorBehavior);

/// Used for benchmarking to access private API of the crate.
#[cfg(feature = "public-internals")]
#[doc(hidden)]
pub mod internals {
    pub use crate::ids::impl_types::*;
    pub use crate::scheduler::internals::*;
    use crate::{GlobalId, GlobalReactionId};

    pub fn new_global_rid(u: GlobalIdImpl) -> GlobalReactionId {
        GlobalReactionId(GlobalId::from_raw(u))
    }
}
