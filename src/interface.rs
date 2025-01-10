use crate::common::Pdu;
use crate::error::ModbusTransportError;
use crate::lib::*;

/// Transport/DataLink layer abstraction
pub trait Transport {
    /// Send a Protocol Data Unit
    fn send(&mut self, pdu: &Pdu)
        -> impl future::Future<Output = Result<(), ModbusTransportError>>;
    /// Receive a Protocol Data Unit
    fn recv(&mut self) -> impl future::Future<Output = Result<Pdu, ModbusTransportError>>;
}
