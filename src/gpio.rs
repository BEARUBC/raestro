#[repr(u8)]
pub enum GpioUartPins {
    TX = 14u8,
    RX = 15u8,
}
#[repr(u32)]
pub enum UartMetaData {
    BAUDRATE = 50u32,
    DATABITS = 8u32,
    STOPBITS = 1u32,
}
pub const BUFFER: [u8; 1usize] = [0x01u8];
