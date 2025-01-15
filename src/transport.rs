use crate::frame::pdu::Pdu;
use crate::lib::*;

#[cfg(feature = "rtu")]
pub mod rtu;

#[cfg(feature = "tcp")]
pub mod tcp;

/// Transport/DataLink layer abstraction
pub trait Transport {
    /// Send a Protocol Data Unit
    fn send(
        &mut self,
        pdu: &Pdu,
    ) -> impl future::Future<Output = Result<(), Box<dyn error::Error + Send + Sync>>>;
    /// Receive a Protocol Data Unit
    fn recv(
        &mut self,
    ) -> impl future::Future<Output = Result<Pdu, Box<dyn error::Error + Send + Sync>>>;
    /// Flush the transport
    fn flush(
        &mut self,
    ) -> impl future::Future<Output = Result<(), Box<dyn error::Error + Send + Sync>>>;
}
