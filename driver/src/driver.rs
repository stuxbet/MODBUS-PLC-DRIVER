// use std::sync::Arc;
use std::{net::SocketAddr, sync::Arc};
use crate::{errors::PLCError, PLCCard};
use client::Context;
use tokio::{self, sync::Mutex};
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
    pub connection:Arc<Mutex<Context>>,
    
}

impl PLCDriver {

    pub async fn connect(config:DriverConfig) -> Result<PLCDriver, PLCError> {
    
        //connect to addresss and return error if it cant
        let context = match tcp::connect(config.address).await {
            Ok(con) => con,
            Err(_) => return Err(PLCError::Initialization("Can not connect to plc".to_string())),
        };

        let driver = PLCDriver{
            config,
            number_of_cards:0,
            cards: Vec::new(),  
            connection:Arc::new(Mutex::new(context))
        };

        Ok(driver)
    }

    pub fn add_card(&mut self, card:PLCCard){
        self.cards.push(card);

        println!("Added card bitch");

        println!("This what we looking at:");
        for given_card in self.cards.clone(){
            println!("{:?}",given_card );
            
        }
        println!("");

    }

}


