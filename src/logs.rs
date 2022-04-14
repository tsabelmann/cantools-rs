use std::fmt::format;
use crate::data::CANData;
use std::path::{Path};
use std::iter::{Iterator, IntoIterator};
use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead, Lines};
use std::str::FromStr;
use std::ops::Div;


pub struct CANDump {
    file: File
}

impl CANDump {
    pub fn open<P>(path: P) -> io::Result<CANDump>
    where
        P: AsRef<Path>
    {
        let file = File::open(path)?;
        Ok(CANDump {file})
    }

    pub fn into_inner(self) -> File {
        self.file
    }
}

#[derive(Debug, PartialEq)]
pub struct CANDumpEntry {
    interface: String,
    can_id: u32,
    data: Vec<u8>
}

impl CANData for CANDumpEntry {
    fn data(&self) -> &[u8] {
        &self.data
    }
    fn dlc(&self) -> usize {
        self.data.len()
    }
}

impl CANDumpEntry {
    pub fn new(interface: &str, can_id:u32, data: Vec<u8>) -> Self {
        CANDumpEntry {
            interface: String::from(interface),
            can_id,
            data
        }
    }
}

pub enum CANDumpEntryParseError {
    MissingInterfaceData,
    MissingCanIdData,
    MissingDlcData,
    ParseDlcError,
    ParseCanIdError,
    ParseCanDataError,
    DlcDataError
}

impl FromStr for CANDumpEntry {
    type Err = CANDumpEntryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits = s
            .split(' ')
            .collect::<Vec<_>>();

        let interface = match splits.get(0).copied() {
            Some(interface) => interface,
            None => return Err(CANDumpEntryParseError::MissingInterfaceData)
        };

        let can_id = match splits
            .get(1).copied().map(|e| u32::from_str_radix(e, 16)) {
            Some(Ok(can_id)) => can_id,
            _ => return Err(CANDumpEntryParseError::MissingCanIdData)
        };

        let mut data = Vec::new();
        for entry in splits.into_iter().skip(3) {
            match u8::from_str_radix(entry, 16) {
                Ok(u8_entry) => data.push(u8_entry),
                _ => return Err(CANDumpEntryParseError::ParseCanIdError)
            }
        }

        Ok(CANDumpEntry::new(interface, can_id, data))
    }
}

impl ToString for CANDumpEntry {
    fn to_string(&self) -> String {
        let data_string = self.data.iter()
            .map(|x| format!("{:02X}", x))
            .collect::<Vec<_>>()
            .join(" ");

        format!("{} {:08X} [{}] {}",
                self.interface,
                self.can_id,
                self.data.len(),
                data_string)
    }
}

pub struct CANDumpIterator {
    lines: Lines<BufReader<File>>
}

impl Iterator for CANDumpIterator {
    type Item = CANDumpEntry;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = self.lines.next();
            match line {
                Some(Ok(line)) => {
                    match line.parse::<Self::Item>() {
                        Ok(entry) => return Some(entry),
                        Err(_) => continue
                    }
                },
                Some(Err(_)) => continue,
                None => return None
            }
        }
    }
}

impl IntoIterator for CANDump {
    type Item = CANDumpEntry;
    type IntoIter = CANDumpIterator;
    fn into_iter(self) -> Self::IntoIter {
        CANDumpIterator {
            lines: io::BufReader::new(self.into_inner()).lines()
        }
    }
}

pub struct CANDumpLog {
    file: File
}

impl CANDumpLog {
    pub fn open<P>(path: P) -> io::Result<CANDumpLog>
    where
        P: AsRef<Path>
    {
        let file = File::open(path)?;
        Ok(CANDumpLog {file})
    }

    pub fn into_inner(self) -> File {
        self.file
    }
}

#[derive(Debug, PartialEq)]
pub struct CANDumpLogEntry {
    timestamp: f64,
    interface: String,
    can_id: u32,
    data: Vec<u8>,
    flag: Option<u8>
}

impl CANDumpLogEntry {
    pub fn new(timestamp: f64, interface: &str, can_id:u32, data: Vec<u8>, flag: Option<u8>) -> Self {
        CANDumpLogEntry {
            timestamp,
            interface: String::from(interface),
            can_id,
            data,
            flag
        }
    }
}

pub enum CANDumpLogEntryParseError {
    MissingTimestampData,
    ParseTimestampError,
    MissingInterfaceData,
    MissingCompoundCanData,
    MissingCanIdData,
    MissingCanData,
    MissingFlagData,
    ParseCanIdError,
    ParseCanDataError,
    ParseFlagError,
    Unspecified
}

impl FromStr for CANDumpLogEntry {
    type Err = CANDumpLogEntryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits = s.split(' ').take(3).collect::<Vec<_>>();

        let timestamp = match splits.get(0).copied() {
            Some(timestamp) => timestamp,
            None => return Err(CANDumpLogEntryParseError::MissingTimestampData)
        };

        let timestamp = match timestamp[1..timestamp.len() - 1].parse::<f64>() {
            Ok(timestamp) => timestamp,
            Err(_) => return Err(CANDumpLogEntryParseError::ParseTimestampError)
        };

        let interface = match splits.get(1).copied() {
            Some(interface) => interface,
            None => return Err(CANDumpLogEntryParseError::MissingInterfaceData)
        };

        let can_data = match splits.get(2).copied() {
            Some(can_data) => can_data,
            None => return Err(CANDumpLogEntryParseError::MissingCompoundCanData)
        };

        let can_data_splits = can_data.split('#')
            .take(3)
            .collect::<Vec<_>>();

        return match can_data_splits.len() {
            2 => {
                let can_id_string = match can_data_splits.get(0).copied() {
                    Some(can_id_string) => can_id_string,
                    None => return Err(CANDumpLogEntryParseError::MissingCanIdData)
                };

                let can_id = match u32::from_str_radix(can_id_string, 16) {
                    Ok(can_id) => can_id,
                    Err(_) => return Err(CANDumpLogEntryParseError::ParseCanIdError)
                };

                let data_string = match can_data_splits.get(1).copied() {
                    Some(data_string) => data_string,
                    None => return Err(CANDumpLogEntryParseError::MissingCanData)
                };

                let mut data = Vec::new();
                for i in 0..data_string.len().div(2) {
                    match u8::from_str_radix(&data_string[2 * i..2 * i + 2], 16) {
                        Ok(value) => data.push(value),
                        Err(_) => return Err(CANDumpLogEntryParseError::ParseCanDataError)
                    };
                }

                Ok(CANDumpLogEntry::new(timestamp, interface, can_id, data, None))
            },
            3 => {
                let can_id_string = match can_data_splits.get(0).copied() {
                    Some(can_id_string) => can_id_string,
                    None => return Err(CANDumpLogEntryParseError::MissingCanIdData)
                };

                let can_id = match u32::from_str_radix(can_id_string, 16) {
                    Ok(can_id) => can_id,
                    Err(_) => return Err(CANDumpLogEntryParseError::ParseCanIdError)
                };

                let data_string = match can_data_splits.get(2).copied() {
                    Some(data_string) => data_string,
                    None => return Err(CANDumpLogEntryParseError::MissingCanData)
                };

                let flag_string = match data_string.get(0..1) {
                    Some(flag_string) => flag_string,
                    None => return Err(CANDumpLogEntryParseError::MissingFlagData)
                };

                let flag = match u8::from_str_radix(flag_string, 16) {
                    Ok(flag) => flag,
                    Err(_) => return Err(CANDumpLogEntryParseError::ParseFlagError)
                };

                let mut data = Vec::new();
                for i in 0..(data_string.len() - 1).div(2) {
                    match u8::from_str_radix(&data_string[2 * i + 1..2 * i + 2 + 1], 16) {
                        Ok(value) => data.push(value),
                        Err(_) => return Err(CANDumpLogEntryParseError::ParseCanDataError)
                    };
                }

                Ok(CANDumpLogEntry::new(timestamp, interface, can_id, data, Some(flag)))
            },
            _ => Err(CANDumpLogEntryParseError::Unspecified)
        }
    }
}

impl ToString for CANDumpLogEntry {
    fn to_string(&self) -> String {
        let data_string = self.data
            .iter()
            .map(|x| format!("{:02X}", x))
            .collect::<Vec<_>>()
            .join("");

        return match self.flag {
            Some(flag) => {
                format!("({}) {} {:08X}##{:1X}{}",
                        self.timestamp,
                        self.interface,
                        self.can_id,
                        flag,
                        data_string)
            },
            None => {
                format!("({}) {} {:08X}#{}",
                        self.timestamp,
                        self.interface,
                        self.can_id,
                        data_string)
            }
        }
    }
}

impl CANData for CANDumpLogEntry {
    fn data(&self) -> &[u8] {
        &self.data
    }

    fn dlc(&self) -> usize {
        self.data.len()
    }
}

pub struct CANDumpLogIterator {
    lines: Lines<BufReader<File>>
}

impl Iterator for CANDumpLogIterator {
    type Item = CANDumpLogEntry;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = self.lines.next();
            match line {
                Some(Ok(line)) => {
                    match line.parse::<CANDumpLogEntry>() {
                        Ok(entry) => return Some(entry),
                        Err(_) => continue
                    }
                },
                Some(Err(_)) => continue,
                None => return None
            }
        }
    }
}

impl IntoIterator for CANDumpLog {
    type Item = CANDumpLogEntry;
    type IntoIter = CANDumpLogIterator;
    fn into_iter(self) -> Self::IntoIter {
        CANDumpLogIterator {
            lines: io::BufReader::new(self.into_inner()).lines()
        }
    }
}