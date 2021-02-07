#[repr(u8)]
pub(crate) enum GpioUartPins {
    TX = 14u8,
    RX = 15u8,
}
#[repr(u32)]
pub(crate) enum UartMetaData {
    BAUDRATE = 50u32,
    DATABITS = 8u32,
    STOPBITS = 1u32,
}
