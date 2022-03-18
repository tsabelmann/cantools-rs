use cantools::logs::{CANDump, CANDumpEntry, CANDumpLog, CANDumpLogEntry};

#[test]
fn can_dump_raw_empty() {
    let candump = CANDump::open("candump/raw/empty.log").unwrap();
    let mut iterator = candump.into_iter();
    assert_eq!(iterator.next(), None);
}

#[test]
fn can_dump_raw_once_1() {
    let candump = CANDump::open("candump/raw/once_1.log").unwrap();
    let mut iterator = candump.into_iter();
    let vec = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    assert_eq!(iterator.next(), Some(CANDumpEntry::new("vcan0",
                                                          0x1337,
                                                          vec)));
    assert_eq!(iterator.next(), None);
}

#[test]
fn can_dump_raw_once_2() {
    let candump = CANDump::open("candump/raw/once_2.log").unwrap();
    let mut iterator = candump.into_iter();
    let vec = vec![0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01];
    assert_eq!(iterator.next(), Some(CANDumpEntry::new("can0",
                                                          0x1337,
                                                          vec)));
    assert_eq!(iterator.next(), None);
}

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
    assert_eq!(iterator.next(), Some(CANDumpLogEntry::new(1647037105.079609,
                                                          "vcan0",
                                                          0x42,
                                                          vec![0x12],
                                                          None)));
    assert_eq!(iterator.next(), None);
}

#[test]
fn can_dump_log_once_002() {
    let candump = CANDumpLog::open("candump/logs/once_2.log").unwrap();
    let mut iterator = candump.into_iter();
    assert_eq!(iterator.next(), Some(CANDumpLogEntry::new(1647037105.079609,
                                                          "vcan0",
                                                          0x42,
                                                          vec![0x12],
                                                          Some(0xA))));
    assert_eq!(iterator.next(), None);
}

