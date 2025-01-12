use crate::common::Pdu;
use crate::error::ModbusTransportError;
use crate::{interface::Transport, Result};

use super::model::request::*;
use super::model::response::*;
use super::model::Response;

/// Modbus client handler
pub struct Client<T: Transport> {
    transport: T,
}

impl<T: Transport> Client<T> {
    pub fn new(transport: T) -> Self {
        Self { transport }
    }

    pub async fn read_coils(
        &mut self,
        starting_address: u16,
        quantity_of_coils: u16,
    ) -> Result<ReadCoilsResponse> {
        let read_coils = ReadCoilsRequest::new(starting_address, quantity_of_coils)?;
        let response = self.send_request(&read_coils.into_inner()).await?;

        Ok(Response::try_from(response)?)
    }

    pub async fn read_discrete_inputs(
        &mut self,
        starting_address: u16,
        quantity_of_inputs: u16,
    ) -> Result<ReadDiscreteInputsResponse> {
        let read_discrete_inputs =
            ReadDiscreteInputsRequest::new(starting_address, quantity_of_inputs)?;
        let response = self
            .send_request(&read_discrete_inputs.into_inner())
            .await?;

        Ok(Response::try_from(response)?)
    }

    pub async fn read_holding_registers(
        &mut self,
        starting_address: u16,
        quantity_of_registers: u16,
    ) -> Result<ReadHoldingRegistersResponse> {
        let read_holding_registers =
            ReadHoldingRegistersRequest::new(starting_address, quantity_of_registers)?;
        let response = self
            .send_request(&read_holding_registers.into_inner())
            .await?;

        Ok(Response::try_from(response)?)
    }

    pub async fn read_input_registers(
        &mut self,
        starting_address: u16,
        quantity_of_registers: u16,
    ) -> Result<ReadInputRegistersResponse> {
        let read_input_registers =
            ReadInputRegistersRequest::new(starting_address, quantity_of_registers)?;
        let response = self
            .send_request(&read_input_registers.into_inner())
            .await?;

        Ok(Response::try_from(response)?)
    }

    pub async fn write_single_coil(
        &mut self,
        output_address: u16,
        output_value: bool,
    ) -> Result<WriteSingleCoilResponse> {
        let write_single_coil = WriteSingleCoilRequest::new(output_address, output_value)?;
        let response = self.send_request(&write_single_coil.into_inner()).await?;

        Ok(Response::try_from(response)?)
    }

    pub async fn write_single_register(
        &mut self,
        register_address: u16,
        register_value: u16,
    ) -> Result<WriteSingleRegisterResponse> {
        let write_single_register =
            WriteSingleRegisterRequest::new(register_address, register_value)?;
        let response = self
            .send_request(&write_single_register.into_inner())
            .await?;

        Ok(Response::try_from(response)?)
    }

    pub async fn user_defined(
        &mut self,
        function_code: u8,
        data: &[u8],
    ) -> Result<UserDefinedResponse> {
        let user_defined = UserDefinedRequest::new(function_code, data)?;
        let response = self.send_request(&user_defined.into_inner()).await?;

        Ok(Response::try_from((response, function_code))?)
    }

    async fn send_request(&mut self, pdu: &Pdu) -> Result<Pdu> {
        self.transport
            .send(pdu)
            .await
            .map_err(|e| ModbusTransportError::TransportError(e.into()))?;
        let response = self
            .transport
            .recv()
            .await
            .map_err(|e| ModbusTransportError::TransportError(e.into()))?;

        Ok(response)
    }
}

#[cfg(feature = "rtu")]
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_app_client_read_registers() {
        use crate::transport::rtu::SerialTransport;

        let mut transport = SerialTransport::builder("/dev/ttyCH341USB0", 115_200)
            .set_parity(tokio_serial::Parity::None)
            .build()
            .unwrap();

        transport.set_slave_addr(0x50).unwrap();

        let mut client = Client::new(transport);

        let response = client
            .read_holding_registers(0x30, 13)
            .await
            .expect("Failed to read holding registers");

        println!("{}", response);

        let response = client.write_single_register(0x1F, 0x0004).await.unwrap();

        println!("{}", response);
    }
}
