use crate::maestro::intrinsics::MaestroError;

#[test]
fn no_errors() -> () {
    let err = 0u16;
    let actual_vec = MaestroError::from_data(err);

    assert_eq!(actual_vec.len(), 0usize);
}

#[test]
fn ser_signal_error() -> () {
    let err = 1u16;
    let actual_vec = MaestroError::from_data(err);

    assert_eq!(actual_vec.len(), 1usize);
    assert_eq!(actual_vec[0usize], MaestroError::SerSignalError);
}

#[test]
fn ser_overrun_error() -> () {
    let err = 2u16;
    let actual_vec = MaestroError::from_data(err);

    assert_eq!(actual_vec.len(), 1usize);
    assert_eq!(actual_vec[0usize], MaestroError::SerOverrunError);
}

#[test]
fn two_errors() -> () {
    let err = 3u16;
    let actual_vec = MaestroError::from_data(err);

    assert_eq!(actual_vec.len(), 2usize);
    assert_eq!(actual_vec[0usize], MaestroError::SerSignalError);
    assert_eq!(actual_vec[1usize], MaestroError::SerOverrunError);
}

#[test]
fn invalid_err() -> () {
    let err = 0x0200u16;
    let actual_vec = MaestroError::from_data(err);

    assert_eq!(actual_vec.len(), 0usize);
}

#[test]
fn all_errors() -> () {
    let err = 0x01ffu16;
    let actual_vec = MaestroError::from_data(err);

    assert_eq!(actual_vec.len(), 9usize);
    assert_eq!(actual_vec[0usize], MaestroError::SerSignalError);
    assert_eq!(actual_vec[1usize], MaestroError::SerOverrunError);
    assert_eq!(actual_vec[2usize], MaestroError::SerBufferFull);
    assert_eq!(actual_vec[3usize], MaestroError::SerCrcError);
    assert_eq!(actual_vec[4usize], MaestroError::SerProtocolError);
    assert_eq!(actual_vec[5usize], MaestroError::SerTimeout);
    assert_eq!(actual_vec[6usize], MaestroError::ScriptStackError);
    assert_eq!(actual_vec[7usize], MaestroError::ScriptCallStackError);
    assert_eq!(actual_vec[8usize], MaestroError::ScriptPcError);
}
