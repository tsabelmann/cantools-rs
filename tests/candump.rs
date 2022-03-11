use cantools::logs::{CANDumpLog, CANDumpLogEntry};

#[test]
fn candump_log_empty() {
    let candump = CANDumpLog::open("candump/logs/empty.log").unwrap();
    let mut iterator = candump.into_iter();
    assert_eq!(iterator.next(), None);
}

#[test]
fn candump_log_once_001() {
    let candump = CANDumpLog::open("candump/logs/once.log").unwrap();
    let mut iterator = candump.into_iter();
    assert_eq!(iterator.next(), Some(CANDumpLogEntry::new(1647037105.079609, "vcan0", 0x42, &[0x12], None)));
    assert_eq!(iterator.next(), None);
}

#[test]
fn candump_log_once_002() {
    let candump = CANDumpLog::open("candump/logs/once_1.log").unwrap();
    let mut iterator = candump.into_iter();
    assert_eq!(iterator.next(), Some(CANDumpLogEntry::new(1647037105.079609, "vcan0", 0x42, &[0x12], Some(0xA))));
    assert_eq!(iterator.next(), None);
}

