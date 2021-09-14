use std::fmt::{Display, Formatter};

use crate::PortId;

/// An error occurring during initialization of the reactor program.
/// Should never occur unless the graph is built by hand, and not
/// by a Lingua Franca compiler.
pub enum AssemblyError {
    CyclicDependency(PortId, PortId),
    CannotBind(PortId, PortId),
    CannotSet(PortId),
}

impl Display for AssemblyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssemblyError::CyclicDependency(upstream, downstream) => write!(f, "Port {} is already in the downstream of port {}", upstream, downstream),
            AssemblyError::CannotBind(upstream, downstream) => write!(f, "Cannot bind {} to {}, downstream is already bound", upstream, downstream),
            AssemblyError::CannotSet(port) => write!(f, "Cannot set {} explicitly as it is bound", port),
        }
    }
}
