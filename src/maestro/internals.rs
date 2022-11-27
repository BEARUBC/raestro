#![allow(unused)]

pub(super) const BUFFER_SIZE: usize = 6usize;
pub(super) const SYNC: u8 = 0xaau8;
pub(super) const DEVICE_NUMBER: u8 = 0x0cu8;
pub(super) const DATA_BITS: u8 = 8u8;
pub(super) const STOP_BITS: u8 = 1u8;
pub(super) const MIN_WRITE_LENGTH: usize = 3usize;
pub(super) const RESPONSE_SIZE: u8 = 2u8;

/// ### Purpose:
/// All available command flags supported by the `Pololu Protocol`.
#[derive(Copy, Clone, PartialEq)]
#[cfg_attr(test, derive(Debug))]
#[repr(u8)]
pub(super) enum CommandFlags {
    SetTarget = 0x84u8,
    SetSpeed = 0x87u8,
    SetAcceleration = 0x89u8,
    GetPosition = 0x90u8,
    GetErrors = 0xA1u8,
    GoHome = 0xA2u8,
    StopScript = 0xA4u8,
    RestartScriptAtSubRoutine = 0xA7u8,
    RestartScriptAtSubRoutineWithParameter = 0xA8u8,
    GetScriptStatus = 0xAEu8,
}
