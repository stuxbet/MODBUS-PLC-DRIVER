// use std::sync::Arc;
use std::net::SocketAddr;
use crate::errors::PLCError;
use tokio;
use tokio_modbus::prelude::*;


#[derive(Debug, Clone, PartialEq)]
pub struct  DriverConfig {
    pub address:SocketAddr,
    //this is how often we check for input and will be a value of hertz
    pub cycle_speed:u64

}

#[derive(Debug, Clone)]
pub struct PLCDriver {
    pub config:DriverConfig,
    pub number_of_cards:u32,
    pub cards:Vec<PLCCard>,
}

impl PLCDriver {

    pub async fn connect(config:DriverConfig) -> Result<PLCDriver, PLCError> {
    
        //connect to addresss
        let mut context = tcp::connect(config.address).await.unwrap();

        let driver = PLCDriver{
                config,
                number_of_cards:0,
                cards: Vec::new(),   
        };

        Ok(driver)
    }

    pub fn add_card(self){
        //this method will append a card to our driver
    }

}


#[derive(Debug, Clone)]
pub enum PLCCard{
    AnalogInput(AnalogInputCard),
    AnalogOutput(AnalogOutputCard),
    DigitalInput(DigitalInputCard),
    DigitalOutput(DigitalOutputCard)
}
#[derive(Debug, Clone)]
pub struct AnalogInputCard {}
#[derive(Debug, Clone)]
pub struct AnalogOutputCard {}
#[derive(Debug, Clone)]
pub struct DigitalInputCard {}
#[derive(Debug, Clone)]
pub struct DigitalOutputCard {}
