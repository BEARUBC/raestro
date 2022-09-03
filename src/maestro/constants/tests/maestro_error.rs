use crate::maestro::constants::ErrorValues;

#[test]
fn no_errors() -> () {
    let err = 0u16;
    let actual_vec = ErrorValues::from_data(err);

    assert_eq!(actual_vec.len(), 0usize);
}

#[test]
fn ser_signal_error() -> () {
    let err = 1u16;
    let actual_vec = ErrorValues::from_data(err);

    assert_eq!(actual_vec.len(), 1usize);
    assert_eq!(actual_vec[0usize], ErrorValues::SerSignalError);
}

#[test]
fn ser_overrun_error() -> () {
    let err = 2u16;
    let actual_vec = ErrorValues::from_data(err);

    assert_eq!(actual_vec.len(), 1usize);
    assert_eq!(actual_vec[0usize], ErrorValues::SerOverrunError);
}

#[test]
fn two_errors() -> () {
    let err = 3u16;
    let actual_vec = ErrorValues::from_data(err);

    assert_eq!(actual_vec.len(), 2usize);
    assert_eq!(actual_vec[0usize], ErrorValues::SerSignalError);
    assert_eq!(actual_vec[1usize], ErrorValues::SerOverrunError);
}

#[test]
fn invalid_err() -> () {
    let err = 0x0200u16;
    let actual_vec = ErrorValues::from_data(err);

    assert_eq!(actual_vec.len(), 0usize);
}

#[test]
fn all_errors() -> () {
    let err = 0x01ffu16;
    let actual_vec = ErrorValues::from_data(err);

    assert_eq!(actual_vec.len(), 9usize);
    assert_eq!(actual_vec[0usize], ErrorValues::SerSignalError);
    assert_eq!(actual_vec[1usize], ErrorValues::SerOverrunError);
    assert_eq!(actual_vec[2usize], ErrorValues::SerBufferFull);
    assert_eq!(actual_vec[3usize], ErrorValues::SerCrcError);
    assert_eq!(actual_vec[4usize], ErrorValues::SerProtocolError);
    assert_eq!(actual_vec[5usize], ErrorValues::SerTimeout);
    assert_eq!(actual_vec[6usize], ErrorValues::ScriptStackError);
    assert_eq!(actual_vec[7usize], ErrorValues::ScriptCallStackError);
    assert_eq!(actual_vec[8usize], ErrorValues::ScriptPcError);
}
