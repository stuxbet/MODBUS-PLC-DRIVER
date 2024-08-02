use std::{error::Error, net::{AddrParseError, SocketAddr}, result};

use driver::{self, DriverConfig, PLCCard, PLCDriver};

#[tokio::main]
async fn main() {

    let address: Result<SocketAddr, AddrParseError> = "10.10.0.101:502".parse();
    // let addr: SocketAddr = SERVER_HOST.parse()?;


    let config = DriverConfig{
        address:address.unwrap(),
        cycle_speed:20,
    };
    let driver = PLCDriver::connect(config).await;

    let mut driver = driver.unwrap();    
    driver.add_card(PLCCard::AnalogOutput(driver::AnalogOutputCard{}));
    driver.add_card(PLCCard::AnalogOutput(driver::AnalogOutputCard{}));
    driver.add_card(PLCCard::AnalogOutput(driver::AnalogOutputCard{}));
}
