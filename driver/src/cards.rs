
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
