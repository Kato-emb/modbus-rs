use core::time::Duration;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    time::{sleep, Instant},
};
use tokio_serial::{SerialPortBuilder, SerialPortBuilderExt, SerialStream};

use crate::{
    common::Pdu,
    error::{ModbusRtuError, ModbusTransportError},
    interface::Transport,
};

use super::{Adu, Result};

const RTU_BITS_PER_CHAR: u8 = 11;

#[derive(Debug)]
pub(crate) struct RtuContext {
    slave_addr: u8,
    latest_time: Instant,
    t1_5: Duration,
    t3_5: Duration,
}

impl Default for RtuContext {
    fn default() -> Self {
        Self {
            slave_addr: 0,
            latest_time: Instant::now(),
            t1_5: Duration::from_secs(86400),
            t3_5: Duration::from_secs(86400),
        }
    }
}

impl RtuContext {
    pub fn set_interval(&mut self, baud_rate: u32) {
        if baud_rate <= 19200 {
            let sec_per_char = RTU_BITS_PER_CHAR as f64 / baud_rate as f64;

            self.t1_5 = Duration::from_secs_f64(sec_per_char * 1.5);
            self.t3_5 = Duration::from_secs_f64(sec_per_char * 3.5);
        } else {
            self.t1_5 = Duration::from_micros(750);
            self.t3_5 = Duration::from_micros(1750);
        }
    }
}

#[derive(Debug)]
pub struct SerialTransport {
    port: SerialStream,
    ctx: RtuContext,
    buffer: Adu,
}

impl SerialTransport {
    pub fn builder<P: AsRef<str>>(path: P, baud_rate: u32) -> SerialTransportBuilder {
        SerialTransportBuilder::new(path, baud_rate)
    }

    /// Set the slave address
    ///
    /// Note. 2.2 MODBUS Addressing rules
    pub fn set_slave_addr(&mut self, slave_addr: u8) -> Result<()> {
        if (1..=247).contains(&slave_addr) {
            self.ctx.slave_addr = slave_addr;
            Ok(())
        } else {
            Err(ModbusRtuError::InvalidSlaveAddress(slave_addr).into())
        }
    }
}

impl Transport for SerialTransport {
    async fn send(&mut self, pdu: &Pdu) -> Result<()> {
        self.buffer.clear();

        // Modbus RTU header
        self.buffer.put_u8(self.ctx.slave_addr)?;
        // Protocol Data Unit
        self.buffer.extend_from_slice(pdu.as_slice())?;
        // CRC
        let crc = calc_crc(self.buffer.as_slice());
        self.buffer.put_u16_le(crc)?;

        self.port.write_all(self.buffer.as_slice()).await?;

        Ok(())
    }

    async fn recv(&mut self) -> Result<Pdu> {
        self.buffer.clear();
        let t3_5_timer = sleep(Duration::from_secs(86400));
        tokio::pin!(t3_5_timer);

        loop {
            tokio::select! {
                res = self.port.read(self.buffer.as_write_slice()) => {
                    let current_time = Instant::now();

                    match res {
                        Ok(n) => {
                            // Check if a silent interval of more than 1.5 character times occurs between two characters
                            if !self.buffer.is_empty() {
                                let elapsed = current_time.duration_since(self.ctx.latest_time);
                                if elapsed > self.ctx.t1_5 {
                                    return Err(ModbusRtuError::FrameIncomplete.into());
                                }
                            }

                            self.buffer.set_position(self.buffer.len() + n);

                            if self.buffer.get(0) == Some(&self.ctx.slave_addr) {
                                // Validate frame if slave address matches
                                let len = self.buffer.len();
                                if len > 2 {
                                    let crc = self.buffer.get_u16_le(len - 2).unwrap();
                                    if checksum(&self.buffer.as_slice()[..len - 2], crc).is_ok() {
                                        return Ok(self.buffer.pdu()?);
                                    }
                                }
                            } else {
                                // Ignore the frame
                                self.buffer.clear();
                            }
                        }
                        Err(ref err) if err.kind() == std::io::ErrorKind::TimedOut && self.buffer.is_empty() => {}
                        Err(err) => return Err(err.into()),
                    }

                    self.ctx.latest_time = current_time;
                    t3_5_timer.as_mut().reset(current_time + self.ctx.t3_5);
                    continue;
                }
                _ = &mut t3_5_timer => {
                    let len = self.buffer.len();
                    if len > 2 {
                        let crc = self.buffer.get_u16_le(len - 2).unwrap();
                        checksum(&self.buffer.as_slice()[..len - 2], crc)?;
                        return Ok(self.buffer.pdu()?);
                    }

                    return Err(ModbusTransportError::Timeout);
                }
            }
        }
    }
}

pub struct SerialTransportBuilder {
    inner: SerialPortBuilder,
    ctx: RtuContext,
}

impl SerialTransportBuilder {
    pub fn new<P: AsRef<str>>(path: P, baud_rate: u32) -> Self {
        let mut ctx = RtuContext::default();
        ctx.set_interval(baud_rate);

        let inner = tokio_serial::new(path.as_ref(), baud_rate)
            .flow_control(tokio_serial::FlowControl::None)
            .stop_bits(tokio_serial::StopBits::One)
            .parity(tokio_serial::Parity::Even)
            .data_bits(tokio_serial::DataBits::Eight)
            .timeout(ctx.t3_5);

        Self { inner, ctx }
    }

    /// Set the number of data bits
    ///
    /// Note. 2.5.1.1 MODBUS Message RTU Framing
    pub fn set_baud_rate(self, baud_rate: u32) -> Self {
        let mut ctx = self.ctx;
        ctx.set_interval(baud_rate);

        let inner = self.inner.baud_rate(baud_rate).timeout(ctx.t3_5);

        Self { inner, ctx }
    }

    /// Set the number of data bits
    ///
    /// Note. 2.5.1 RTU Transmission Mode
    pub fn set_parity(self, parity: tokio_serial::Parity) -> Self {
        let inner = match parity {
            tokio_serial::Parity::Even | tokio_serial::Parity::Odd => self
                .inner
                .stop_bits(tokio_serial::StopBits::One)
                .parity(parity),
            tokio_serial::Parity::None => self
                .inner
                .stop_bits(tokio_serial::StopBits::Two)
                .parity(parity),
        };

        Self {
            inner,
            ctx: self.ctx,
        }
    }

    pub fn build(self) -> Result<SerialTransport> {
        let port = self
            .inner
            .open_native_async()
            .map_err(|err| ModbusRtuError::SerialError(err))?;

        Ok(SerialTransport {
            port,
            ctx: self.ctx,
            buffer: Adu::new(),
        })
    }
}

/// Generate a Modbus RTU frame CRC.
fn calc_crc(data: &[u8]) -> u16 {
    let mut crc = 0xFFFFu16;

    for byte in data {
        crc ^= u16::from(*byte);

        for _ in 0..8 {
            if crc & 0x0001 != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }

    crc
}

fn checksum(data: &[u8], crc: u16) -> Result<()> {
    if calc_crc(data) == crc {
        Ok(())
    } else {
        Err(ModbusTransportError::RtuError(ModbusRtuError::CrcValidationFailure).into())
    }
}

#[cfg(test)]
mod tests {
    use crate::app::model::{
        request::ReadHoldingRegistersRequest, response::ReadHoldingRegistersResponse,
    };

    use super::*;

    #[tokio::test]
    async fn test_transport_rtu_session() {
        let mut transport = SerialTransport::builder("/dev/ttyCH341USB0", 115_200)
            .set_parity(tokio_serial::Parity::None)
            .build()
            .unwrap();

        transport.set_slave_addr(0x50).unwrap();

        for _ in 0..10 {
            let start = Instant::now();
            let request = ReadHoldingRegistersRequest::new(0x34, 9).unwrap();
            println!("{:?}", request);
            transport.send(&request.into_inner()).await.unwrap();

            let res = transport.recv().await.unwrap();
            let response = ReadHoldingRegistersResponse::try_from(res).unwrap();
            println!("{:?} {}", start.elapsed(), response);
        }
    }
}
