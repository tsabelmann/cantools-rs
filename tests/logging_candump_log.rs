use cantools::logging::{CANDumpLog, CANDumpLogEntry};

#[test]
fn can_dump_log_empty() {
    let candump = CANDumpLog::open("candump/logs/empty.log").unwrap();
    let mut iterator = candump.into_iter();
    assert_eq!(iterator.next(), None);
}

#[test]
fn can_dump_log_once_001() {
    let candump = CANDumpLog::open("candump/logs/once_1.log").unwrap();
    let mut iterator = candump.into_iter();

    assert_eq!(
        iterator.next(),
        Some(CANDumpLogEntry::new(1647037105.079609, "vcan0", 0x42, vec![0x12], None).unwrap())
    );
    assert_eq!(iterator.next(), None);
}

#[test]
fn can_dump_log_once_002() {
    let candump = CANDumpLog::open("candump/logs/once_2.log").unwrap();
    let mut iterator = candump.into_iter();
    assert_eq!(
        iterator.next(),
        Some(
            CANDumpLogEntry::new(1647037105.079609, "vcan0", 0x42, vec![0x12], Some(0xA)).unwrap()
        )
    );
    assert_eq!(iterator.next(), None);
}
